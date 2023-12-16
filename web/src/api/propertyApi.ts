import {get} from "../config/http.ts";

interface PropertyType {
    property_id?: number,
    product_id: number,
    identifier: string,
    property_name: string,
    description?: string,
    data_schema: DataSchema,
}

type DataSchema =
    | { Integer: { len: number; unit: string; min: number; max: number; unit_name: string } }
    | { String: { unit: string; unit_name: string } }
    | { VaryString: { len: number; unit: string; unit_name: string } }
    | { Float: { len: number; unit: string; min: number; max: number; unit_name: string } }
    | { Boolean: { bool_false: string; bool_true: string } }
    | { DateTime: null }
    | { Enum: { len: number; enum_detail: Array<[number, string]> } }
    | { Double: { len: number; unit: string; min: number; max: number; unit_name: string } };


// 添加 getType 方法
function getDataType(schema: DataSchema): string {
    if ("Integer" in schema) {
        return "Integer";
    } else if ("String" in schema) {
        return "String";
    } else if ("VaryString" in schema) {
        return "VaryString";
    } else if ("Float" in schema) {
        return "Float";
    } else if ("Boolean" in schema) {
        return "Boolean";
    } else if ("DateTime" in schema) {
        return "DateTime";
    } else if ("Enum" in schema) {
        return "Enum";
    } else if ("Double" in schema) {
        return "Double";
    } else {
        return "Unknown Type";
    }
}

// 添加 getDefinition 方法
function getDefinition(schema: DataSchema): string {
    if ("Integer" in schema) {
        return `取值范围: ${schema.Integer.min}-${schema.Integer.max}`;
    } else if ("String" in schema) {
        return `单位: ${schema.String.unit}`;
    } else if ("VaryString" in schema) {
        return `长度: ${schema.VaryString.len},单位: ${schema.VaryString.unit}`;
    } else if ("Float" in schema) {
        return `取值范围: ${schema.Float.min}-${schema.Float.max}`;
    } else if ("Boolean" in schema) {
        return `false: ${schema.Boolean.bool_false}, true: ${schema.Boolean.bool_true}`;
    } else if ("DateTime" in schema) {
        return "时间";
    } else if ("Enum" in schema) {
        return `${JSON.stringify(schema.Enum.enum_detail)}`;
    } else if ("Double" in schema) {
        return `取值范围: ${schema.Double.min}-${schema.Double.max}`;
    } else {
        return "Unknown Type";
    }
}


/**
 * 分页查询产品
 * @param product_id
 */
async function propertiesList(product_id: number): Promise<PropertyType[]> {
    return await get<PropertyType[]>(`/property/${product_id}`);
}

export {getDataType, getDefinition, propertiesList}
export type {PropertyType, DataSchema}