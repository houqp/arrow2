use std::collections::VecDeque;
use std::io::{Read, Seek};

use gen::Schema::MetadataVersion;

use crate::array::StructArray;
use crate::datatypes::DataType;
use crate::error::Result;
use crate::io::ipc::gen::Message::BodyCompression;

use super::super::super::gen;
use super::super::deserialize::{read, skip, Node};
use super::super::read_basic::*;

#[allow(clippy::too_many_arguments)]
pub fn read_struct<R: Read + Seek>(
    field_nodes: &mut VecDeque<Node>,
    data_type: DataType,
    buffers: &mut VecDeque<&gen::Schema::Buffer>,
    reader: &mut R,
    block_offset: u64,
    is_little_endian: bool,
    compression: Option<BodyCompression>,
    version: MetadataVersion,
) -> Result<StructArray> {
    let field_node = field_nodes.pop_front().unwrap().0;

    let validity = read_validity(
        buffers,
        field_node,
        reader,
        block_offset,
        is_little_endian,
        compression,
    )?;

    let fields = StructArray::get_fields(&data_type);

    let values = fields
        .iter()
        .map(|field| {
            read(
                field_nodes,
                field.data_type().clone(),
                buffers,
                reader,
                block_offset,
                is_little_endian,
                compression,
                version,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(StructArray::from_data(data_type, values, validity))
}

pub fn skip_struct(
    field_nodes: &mut VecDeque<Node>,
    data_type: &DataType,
    buffers: &mut VecDeque<&gen::Schema::Buffer>,
) {
    let _ = field_nodes.pop_front().unwrap();

    let _ = buffers.pop_front().unwrap();

    let fields = StructArray::get_fields(data_type);

    fields
        .iter()
        .for_each(|field| skip(field_nodes, field.data_type(), buffers))
}
