import {del, get, post} from "../config/http.ts";

interface PropertyType {
    property_id?: number,
    product_id: number,
    identifier: string,
    property_name: string,
    description?: string,
    data_schema?: DataSchema,
    dataType?: string,  // 用于前端展示
}

interface Unit {
    id: number,
    unit: string,
    unit_name: string,
    unit_description: string,
}

type DataSchema =
    | { Integer: { len: number; unit: string; min: number; max: number; unit_name: string } }
    | { String: { unit: string; unit_name: string } }
    | { VaryString: { len: number; unit: string; unit_name: string } }
    | { Float: { len: number; unit: string; min: number; max: number; unit_name: string } }
    | { Boolean: { bool_false: string; bool_true: string } }
    | { DateTime: null }
    | {
    Enum: Array<{
        key: string;
        value: string;
    }>
}
    | { Double: { len: number; unit: string; min: number; max: number; unit_name: string } };


// 添加 getType 方法
function getDataType(schema?: DataSchema): string {
    if (schema === undefined) return "Unknown Type";
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
function getDefinition(schema?: DataSchema): string {
    if (schema === undefined) return "Unknown Type";
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
        const res = schema.Enum.map((item) => {
            return `${item.key}: ${item.value}`;
        });
        return res.join(",");
    } else if ("Double" in schema) {
        return `取值范围: ${schema.Double.min}-${schema.Double.max}`;
    } else {
        return "Unknown Type";
    }
}


/**
 * 列表查询
 * @param product_id
 * @param search_param
 */
async function propertiesList(product_id: number, search_param?: string): Promise<PropertyType[]> {
    return await get<PropertyType[]>(`/property`, {
        product_id: product_id,
        search_param: search_param || ''
    });
}

/**
 * 分页查询产品
 * @param property
 */
async function propertyAdd(property: PropertyType): Promise<void> {
    function parseUnitName() {
        const dataSchema = property.data_schema;
        if (dataSchema === undefined) {
            throw new Error("dataSchema is undefined");
        }
        if ("Integer" in dataSchema) {
            const [unit, unit_name] = dataSchema.Integer.unit.split("|")
            dataSchema.Integer.unit = unit;
            dataSchema.Integer.unit_name = unit_name;
            return
        }
        if ("String" in dataSchema) {
            const [unit, unit_name] = dataSchema.String.unit.split("|")
            dataSchema.String.unit = unit;
            dataSchema.String.unit_name = unit_name;
            return
        }
        if ("VaryString" in dataSchema) {
            const [unit, unit_name] = dataSchema.VaryString.unit.split("|")
            dataSchema.VaryString.unit = unit;
            dataSchema.VaryString.unit_name = unit_name;
            return
        }
        if ("Double" in dataSchema) {
            const [unit, unit_name] = dataSchema.Double.unit.split("|")
            dataSchema.Double.unit = unit;
            dataSchema.Double.unit_name = unit_name;
            return
        }
        if ("Float" in dataSchema) {
            const [unit, unit_name] = dataSchema.Float.unit.split("|")
            dataSchema.Float.unit = unit;
            dataSchema.Float.unit_name = unit_name;
            return
        }
    }

    parseUnitName();
    console.log(property)
    return await post<void>(`/property`, property);
}

/**
 * 查询单位列表
 */
async function unitList(): Promise<Unit[]> {
    return await get<Unit[]>("/unit");
}

/**
 * 创建属性
 * @param property
 */
async function propertyCreate(property: PropertyType): Promise<PropertyType[]> {
    return await post<PropertyType[]>(`/property`, property);
}

/**
 * 删除属性
 * @param id
 */
async function propertyDelete(id: number): Promise<void> {
    await del<void>(`/property/${id}`,);
}

export {getDataType, getDefinition, propertiesList, propertyCreate, unitList, propertyAdd, propertyDelete}
export type {PropertyType, DataSchema, Unit}