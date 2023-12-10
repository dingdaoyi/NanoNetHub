interface UserInfo{
    username:string;
    avatar:string;
    token: string;
}
function saveUser(user:UserInfo):void{
    const userString = JSON.stringify(user);
    console.log("用户信息:",userString)
    localStorage.setItem('user', userString);
}
function getUser(): UserInfo | null {
    const userString = localStorage.getItem('user');
    if (userString) {
        return JSON.parse(userString);
    }
    return null;
}

export {getUser, saveUser};
export type {UserInfo};