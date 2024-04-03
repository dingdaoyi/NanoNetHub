import {UserType} from "@/types/api.ts";
import {del, get, post, put} from "@/utils/http.ts";

async function updateUser(params: UserType): Promise<boolean> {
    return await put<boolean>('/user', params);
}

async function addUserApi(params: UserType): Promise<boolean> {
    return await post<boolean>('/user', params);
}
async function deleteUserApi(id: number): Promise<boolean> {
    return await del<boolean>(`/user/${id}`,);
}
async function getUserList(): Promise<UserType[]> {
    return await get<UserType[]>('/user');
}
export {updateUser,getUserList,deleteUserApi,addUserApi}