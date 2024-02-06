import {Avatar, Button, Layout, Menu, Modal, theme} from 'antd';

const {Sider, Content} = Layout;
import {ReactNode, useState} from "react";
import {
    InteractionOutlined,
    MenuFoldOutlined,
    MenuUnfoldOutlined, SendOutlined,
    UserOutlined,
} from "@ant-design/icons";
import {Header} from "antd/es/layout/layout";

import {useNavigate} from "react-router-dom";
import "./layout.less"

function HubLayout(params: { children: ReactNode }) {
    const [collapsed, setCollapsed] = useState(false);
    // const [user, setUser] = useState<string>();

    // useEffect(() => {
    //     const initUser = getToken();
    //     if (initUser) {
    //         setUser(initUser);
    //     }
    // }, []);
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
                    <Avatar style={{backgroundColor: '#fde3cf'}}
                            src={"../assets/logo.svg"}/>
                </div>
                <Menu
                    theme="dark"
                    mode="inline"
                    onClick={menuInfo => navigate(menuInfo.key)}
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
                <Header style={{
                    padding: 0, background: colorBgContainer, display: "flex",
                    justifyContent: "space-between", alignItems: "center"
                }}>
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
                    <Button
                        type="text"
                        icon={<UserOutlined/>}
                        onClick={() => {
                            Modal.confirm({
                                title: '确认退出登录？',
                                onOk() {
                                    localStorage.removeItem("token");
                                    navigate('/login')
                                },
                                onCancel() {
                                    console.log('Cancel');
                                },
                            });
                        }}
                        style={{
                            fontSize: '16px',
                            width: 64,
                            height: 64,
                            alignItems: "end",
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
