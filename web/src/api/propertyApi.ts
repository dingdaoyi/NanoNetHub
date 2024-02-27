import {del, get, post, put} from "@/utils/http.ts";
import {DataSchema, PropertyType, Unit} from "@/types/api.ts";

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

function parseUnitName(property: PropertyType) {
    const dataSchema = property.data_schema;
    if (dataSchema === undefined) {
        return property
    }
    if ("Integer" in dataSchema) {
        const [unit, unit_name] = dataSchema.Integer.unit.split("|")
        dataSchema.Integer.unit = unit;
        dataSchema.Integer.unit_name = unit_name;
        return property
    }
    if ("String" in dataSchema) {
        const [unit, unit_name] = dataSchema.String.unit.split("|")
        dataSchema.String.unit = unit;
        dataSchema.String.unit_name = unit_name;
        return property
    }
    if ("VaryString" in dataSchema) {
        const [unit, unit_name] = dataSchema.VaryString.unit.split("|")
        dataSchema.VaryString.unit = unit;
        dataSchema.VaryString.unit_name = unit_name;
        return property
    }
    if ("Double" in dataSchema) {
        const [unit, unit_name] = dataSchema.Double.unit.split("|")
        dataSchema.Double.unit = unit;
        dataSchema.Double.unit_name = unit_name;
        return property
    }
    return property
}

/**
 * 分页查询产品
 * @param property
 */
async function propertyUpdate(property: PropertyType): Promise<void> {
    return await put<void>(`/property`, parseUnitName(property));
}


/**
 * 分页查询产品
 * @param property
 */
async function propertyAdd(property: PropertyType): Promise<void> {
    return await post<void>(`/property`, parseUnitName(property)!);
}

/**
 * 查询单位列表
 */
async function unitList(): Promise<Unit[]> {
    return await get<Unit[]>("/unit");
}

/**
 * 删除属性
 * @param id
 */
async function propertyDelete(id: number): Promise<void> {
    await del<void>(`/property/${id}`,);
}

export {getDataType, getDefinition, propertiesList, unitList, propertyAdd, propertyDelete, propertyUpdate}