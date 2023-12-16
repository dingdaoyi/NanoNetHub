import {Avatar, Button, Layout, Menu, theme} from 'antd';

const {Sider, Content} = Layout;
import {ReactNode, useEffect, useState} from "react";
import {
    MenuFoldOutlined,
    MenuUnfoldOutlined,
    UserOutlined,
} from "@ant-design/icons";
import {Header} from "antd/es/layout/layout";

import {useNavigate} from "react-router-dom";
import {getUser, UserInfo} from "../store/userInfo.tsx";
import "./layout.less"

function HubLayout(params: { children: ReactNode }) {
    const [collapsed, setCollapsed] = useState(false);
    const [user, setUser] = useState<UserInfo>();

    useEffect(() => {
        const initUser = getUser();
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
                        // {
                        //     key: '/admin/tsl',
                        //     icon: <VideoCameraOutlined/>,
                        //     label: '物模型',
                        // },
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
