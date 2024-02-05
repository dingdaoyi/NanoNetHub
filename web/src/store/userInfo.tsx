interface UserInfo {
    username: string;
    avatar: string;
    token: string;
}

function saveToken(token: string): void {
    console.log("用户信息:", token)
    localStorage.setItem('token', token);
}

function getToken(): string | null {
    return localStorage.getItem('token');
}

export {getToken, saveToken};
export type {UserInfo};