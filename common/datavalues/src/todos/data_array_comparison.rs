// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;

use crate::data_array_cast;
use crate::DataColumn;
use crate::DataType;
use crate::DataValue;
use crate::DataValueComparisonOperator;
use crate::Float32Array;
use crate::Float64Array;
use crate::IGetDataType;
use crate::Int16Array;
use crate::Int32Array;
use crate::Int64Array;
use crate::Int8Array;
use crate::Series;
use crate::StringArray;
use crate::UInt16Array;
use crate::UInt32Array;
use crate::UInt64Array;
use crate::UInt8Array;

pub struct DataArrayComparison;

impl DataArrayComparison {
    #[inline]
    pub fn data_array_comparison_op(
        op: DataValueComparisonOperator,
        left: &DataColumn,
        right: &DataColumn,
    ) -> Result<Series> {
        match (left, right) {
            (DataColumn::Array(left_array), DataColumn::Array(right_array)) => {
                let coercion_type = super::data_type_coercion::equal_coercion(
                    &left_array.get_data_type(),
                    &right_array.get_data_type(),
                )?;
                let left_array = data_array_cast(&left_array, &coercion_type)?;
                let right_array = data_array_cast(&right_array, &coercion_type)?;

                match op {
                    DataValueComparisonOperator::Eq => {
                        arrow_array_op!(&left_array, &right_array, eq)
                    }
                    DataValueComparisonOperator::Lt => {
                        arrow_array_op!(&left_array, &right_array, lt)
                    }
                    DataValueComparisonOperator::LtEq => {
                        arrow_array_op!(&left_array, &right_array, lt_eq)
                    }
                    DataValueComparisonOperator::Gt => {
                        arrow_array_op!(&left_array, &right_array, gt)
                    }
                    DataValueComparisonOperator::GtEq => {
                        arrow_array_op!(&left_array, &right_array, gt_eq)
                    }
                    DataValueComparisonOperator::NotEq => {
                        arrow_array_op!(&left_array, &right_array, neq)
                    }
                    DataValueComparisonOperator::Like => {
                        arrow_array_utf8_op!(&left_array, &right_array, like_utf8)
                    }
                    DataValueComparisonOperator::NotLike => {
                        arrow_array_utf8_op!(&left_array, &right_array, nlike_utf8)
                    }
                }
            }

            (DataColumn::Array(array), DataColumn::Constant(scalar, _)) => {
                let coercion_type = super::data_type_coercion::equal_coercion(
                    &array.get_data_type(),
                    &scalar.data_type(),
                )?;
                let left_array = data_array_cast(&array, &coercion_type)?;
                let right_array = data_array_cast(&scalar.to_series_with_size(1)?, &coercion_type)?;
                let scalar = DataValue::from_array(&right_array, 0)?;

                match op {
                    DataValueComparisonOperator::Eq => {
                        arrow_array_op_scalar!(left_array, scalar, eq)
                    }
                    DataValueComparisonOperator::Lt => {
                        arrow_array_op_scalar!(left_array, scalar, lt)
                    }
                    DataValueComparisonOperator::LtEq => {
                        arrow_array_op_scalar!(left_array, scalar, lt_eq)
                    }
                    DataValueComparisonOperator::Gt => {
                        arrow_array_op_scalar!(left_array, scalar, gt)
                    }
                    DataValueComparisonOperator::GtEq => {
                        arrow_array_op_scalar!(left_array, scalar, gt_eq)
                    }
                    DataValueComparisonOperator::NotEq => {
                        arrow_array_op_scalar!(left_array, scalar, neq)
                    }
                    DataValueComparisonOperator::Like => {
                        arrow_array_utf8_op_scalar!(left_array, scalar, like_utf8)
                    }
                    DataValueComparisonOperator::NotLike => {
                        arrow_array_utf8_op_scalar!(left_array, scalar, nlike_utf8)
                    }
                }
            }

            (DataColumn::Constant(scalar, _), DataColumn::Array(array)) => {
                let coercion_type = super::data_type_coercion::equal_coercion(
                    &array.get_data_type(),
                    &scalar.data_type(),
                )?;
                let left_array = data_array_cast(&scalar.to_series_with_size(1)?, &coercion_type)?;
                let right_array = data_array_cast(&array, &coercion_type)?;
                let scalar = DataValue::from_array(&left_array, 0)?;

                match op {
                    DataValueComparisonOperator::Eq => {
                        arrow_array_op_scalar!(right_array, scalar, eq)
                    }
                    DataValueComparisonOperator::Lt => {
                        arrow_array_op_scalar!(right_array, scalar, gt)
                    }
                    DataValueComparisonOperator::LtEq => {
                        arrow_array_op_scalar!(right_array, scalar, gt_eq)
                    }
                    DataValueComparisonOperator::Gt => {
                        arrow_array_op_scalar!(right_array, scalar, lt)
                    }
                    DataValueComparisonOperator::GtEq => {
                        arrow_array_op_scalar!(right_array, scalar, lt_eq)
                    }
                    DataValueComparisonOperator::NotEq => {
                        arrow_array_op_scalar!(right_array, scalar, neq)
                    }
                    DataValueComparisonOperator::Like => {
                        arrow_array_utf8_op_scalar!(right_array, scalar, like_utf8)
                    }
                    DataValueComparisonOperator::NotLike => {
                        arrow_array_utf8_op_scalar!(right_array, scalar, nlike_utf8)
                    }
                }
            }
            (DataColumn::Constant(left_scala, rows), DataColumn::Constant(right_scalar, _)) => {
                let coercion_type = super::data_type_coercion::equal_coercion(
                    &left_scala.data_type(),
                    &right_scalar.data_type(),
                )?;
                let left_array =
                    data_array_cast(&left_scala.to_series_with_size(*rows)?, &coercion_type)?;
                let right_array =
                    data_array_cast(&right_scalar.to_series_with_size(*rows)?, &coercion_type)?;

                match op {
                    DataValueComparisonOperator::Eq => {
                        arrow_array_op!(&left_array, &right_array, eq)
                    }
                    DataValueComparisonOperator::Lt => {
                        arrow_array_op!(&left_array, &right_array, lt)
                    }
                    DataValueComparisonOperator::LtEq => {
                        arrow_array_op!(&left_array, &right_array, lt_eq)
                    }
                    DataValueComparisonOperator::Gt => {
                        arrow_array_op!(&left_array, &right_array, gt)
                    }
                    DataValueComparisonOperator::GtEq => {
                        arrow_array_op!(&left_array, &right_array, gt_eq)
                    }
                    DataValueComparisonOperator::NotEq => {
                        arrow_array_op!(&left_array, &right_array, neq)
                    }
                    DataValueComparisonOperator::Like => {
                        arrow_array_utf8_op!(&left_array, &right_array, like_utf8)
                    }
                    DataValueComparisonOperator::NotLike => {
                        arrow_array_utf8_op!(&left_array, &right_array, nlike_utf8)
                    }
                }
            }
        }
    }
}
