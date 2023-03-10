// Copyright 2023 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::sync::Arc;

use common_catalog::plan::VirtualColumn;
use common_catalog::plan::VirtualColumnMeta;
use common_exception::Result;
use common_expression::BlockMetaInfoDowncast;
use common_expression::DataBlock;
use common_expression::FieldIndex;
use common_pipeline_core::processors::port::InputPort;
use storages_common_pruner::BlockMetaIndex;

use crate::pipelines::processors::port::OutputPort;
use crate::pipelines::processors::processor::Event;
use crate::pipelines::processors::Processor;

pub struct FillVirtualColumnProcessor {
    virtual_columns: BTreeMap<FieldIndex, VirtualColumn>,
    data_blocks: VecDeque<(BlockMetaIndex, DataBlock)>,
    input: Arc<InputPort>,
    output: Arc<OutputPort>,
    output_data: Option<DataBlock>,
}

impl FillVirtualColumnProcessor {
    pub fn create(
        virtual_columns: BTreeMap<FieldIndex, VirtualColumn>,
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
    ) -> Self {
        Self {
            virtual_columns,
            data_blocks: VecDeque::new(),
            input,
            output,
            output_data: None,
        }
    }
}

#[async_trait::async_trait]
impl Processor for FillVirtualColumnProcessor {
    fn name(&self) -> String {
        "FillVirtualColumnProcessor".to_string()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event(&mut self) -> Result<Event> {
        if self.output.is_finished() {
            self.input.finish();
            return Ok(Event::Finished);
        }

        if !self.output.can_push() {
            self.input.set_not_need_data();
            return Ok(Event::NeedConsume);
        }

        if let Some(data_block) = self.output_data.take() {
            self.output.push_data(Ok(data_block));
            return Ok(Event::NeedConsume);
        }

        if self.input.has_data() {
            let mut data_block = self.input.pull_data().unwrap()?;
            if let Some(source_meta) = data_block.take_meta() {
                if let Some(block_meta) = BlockMetaIndex::downcast_from(source_meta) {
                    self.data_blocks.push_back((block_meta, data_block));
                    return Ok(Event::Sync);
                }
            }

            unreachable!();
        }

        if self.input.is_finished() {
            self.output.finish();
            return Ok(Event::Finished);
        }

        self.input.set_need_data();
        Ok(Event::NeedData)
    }

    fn process(&mut self) -> Result<()> {
        if let Some((block_meta, data_block)) = self.data_blocks.front_mut() {
            let num_rows = data_block.num_rows();
            let virtual_column_meta = VirtualColumnMeta {
                segment_id: block_meta.segment_id,
                block_id: block_meta.block_id,
                block_location: block_meta.block_location.clone(),
                segment_location: block_meta.segment_location.clone(),
                snapshot_location: block_meta.snapshot_location.clone().unwrap(),
            };
            for virtual_column in self.virtual_columns.values() {
                let column = virtual_column.generate_column_values(&virtual_column_meta, num_rows);
                data_block.add_column(column);
            }
            // output datablock MUST with empty meta
            self.output_data = Some(DataBlock::new(
                data_block.columns().to_vec(),
                data_block.num_rows(),
            ));
        }
        Ok(())
    }
}
