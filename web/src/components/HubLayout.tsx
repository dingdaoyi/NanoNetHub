import {Avatar, Button, Layout, Menu, theme} from 'antd';

const {Sider, Content} = Layout;
import {ReactNode, useEffect, useState} from "react";
import {
    InteractionOutlined,
    MenuFoldOutlined,
    MenuUnfoldOutlined, SendOutlined,
    UserOutlined,
} from "@ant-design/icons";
import {Header} from "antd/es/layout/layout";

import {useNavigate} from "react-router-dom";
import {getToken, UserInfo} from "../store/userInfo.tsx";
import "./layout.less"

function HubLayout(params: { children: ReactNode }) {
    const [collapsed, setCollapsed] = useState(false);
    const [user, setUser] = useState<UserInfo>();

    useEffect(() => {
        const initUser = getToken();
        if (initUser) {
            setUser(initUser);
        }
    }, []);
    const navigate = useNavigate()
    const {
        token: {colorBgContainer},
    } = theme.useToken();

    return (
        <Layout style={{
            width: "100vw",
            height: "100vh"
        }}>
            <Sider trigger={null} collapsible collapsed={collapsed}>
                <div className="logo-vertical">
                    <Avatar style={{backgroundColor: '#fde3cf', color: '#f56a00'}}
                            src={"../assets/react.svg"}/>
                </div>
                <Menu
                    theme="dark"
                    mode="inline"
                    onClick={menuInfo => {
                        console.log(menuInfo.key)
                        console.log(user?.username)
                        navigate(menuInfo.key)
                    }}
                    defaultSelectedKeys={['1']}
                    items={[
                        {
                            key: '/admin/product',
                            icon: <UserOutlined/>,
                            label: '产品管理',
                        },
                        {
                            key: '/admin/device',
                            icon: <SendOutlined/>,
                            label: '设备管理',
                        },
                        {
                            key: '/system',
                            icon: <SendOutlined/>,
                            label: '系统管理',
                            children: [
                                {
                                    key: '/system/icon',
                                    label: '图标管理',
                                    icon: <InteractionOutlined/>
                                },
                                {
                                    key: '/system/user',
                                    label: '用户管理',
                                },
                            ]
                        },
                    ]}
                />
            </Sider>
            <Layout>
                <Header style={{padding: 0, background: colorBgContainer}}>
                    <Button
                        type="text"
                        icon={collapsed ? <MenuUnfoldOutlined/> : <MenuFoldOutlined/>}
                        onClick={() => setCollapsed(!collapsed)}
                        style={{
                            fontSize: '16px',
                            width: 64,
                            height: 64,
                        }}
                    />
                </Header>
                <Content
                    style={{
                        margin: '24px 16px',
                        padding: 24,
                        minHeight: 280,
                        background: colorBgContainer,
                    }}
                >
                    {params.children}
                </Content>
            </Layout>
        </Layout>
    )
}

export default HubLayout
