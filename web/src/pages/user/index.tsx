import {Button, Form, Input, message, Modal, Space, Table} from "antd";
import {DeleteOutlined, PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {useEffect, useState} from "react";
import { UserType} from "@/types/api.ts";
import {ColumnsType} from "antd/es/table";
import {addUserApi, deleteUserApi, getUserList} from "@/api/userApi.ts";

function UserPage() {
    const [listData, setListData] = useState<UserType[]>([])
    const [loading, setLoading] = useState(false)
    const [editeUser, setEditeUser] = useState<UserType>({
        username:'',
        email:'',
        name:''
    })
    const [username, setUsername] = useState('')
    const [editModalVisible, setEditModalVisible] = useState(false)

    function loadData() {
        setLoading(true)
        getUserList().then((res) => {
            setListData(res)
            setLoading(false)
        })
    }

    useEffect(() => {
        loadData();
    }, []);

    function deleteUser(id: number) {
        Modal.confirm({
            title: '删除用户',
            content: '确定删除该用户吗？',
            okText: '确认',
            cancelText: '取消',
            onOk: async () => {
                deleteUserApi(id)
                    .then(() => {
                        handleSearch()
                    })
            },
        });

    }

    const columns: ColumnsType<UserType> = [
        {
            title: '名称',
            dataIndex: 'name',
            key: 'name',
        },
        {
            title: '账号',
            dataIndex: 'username',
            key: 'username',
        },
        {
            title: '邮箱',
            dataIndex: 'email',
            key: 'email',
        },
        {
            title: '操作',
            key: 'action',
            render: (_, record:UserType) => (
               (<Space size="middle">
                    <Button type="primary" icon={<DeleteOutlined/>} onClick={() => deleteUser(record.id!)}>删除</Button>
                </Space>)
            ),
        },
    ];

    function handleSearch() {
        loadData()
    }

    function handleAdd() {
        setEditModalVisible(true)
    }


    async function handleEditeSubmit() {
        if (editeUser?.name == undefined) {
            message.warning("请输入用户名称")
            return
        }
        const user = {
            name: editeUser?.name,
            email: editeUser.email,
            username:editeUser.username,
        }
        setLoading(false)
        addUserApi(user).then(() => {
            setLoading(false)
            setEditModalVisible(false)
            loadData()
        })
    }

    return (<>
        <Space align="center" style={{
            marginBottom: 16,
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center'
        }}>
            <Space>
                <Input
                    placeholder="用户名称"
                    value={username}
                    width="200px"
                    onChange={(e) => setUsername(e.target.value)}
                />
                <Button type="primary" icon={<SearchOutlined/>} onClick={handleSearch}>搜索</Button>
            </Space>
            <Button type="primary" icon={<PlusOutlined/>} onClick={handleAdd}>新增</Button>
        </Space>
        <Table columns={columns}
               dataSource={listData}
               loading={loading}
               rowKey="id"
               pagination={false}
        />
        <Modal
            title={ "新增用户"}
            open={editModalVisible}
            destroyOnClose={true}
            onCancel={() => setEditModalVisible(false)}
            onOk={async () => {
                await handleEditeSubmit();
            }}
        >
            <Form>
                <Form.Item label="账号">
                    <Input name="name" defaultValue={editeUser?.name}
                           onChange={(value) => {
                               editeUser.name=value.target.value
                               setEditeUser(editeUser)
                           }}/>
                </Form.Item>
                <Form.Item label="用户名称">
                    <Input name="username" defaultValue={editeUser?.username}
                           onChange={(value) => {
                               editeUser.username=value.target.value
                               setEditeUser(editeUser)
                           }}/>
                </Form.Item>
                <Form.Item label="邮箱">
                    <Input name="email" defaultValue={editeUser?.email}
                           onChange={(value) => {
                               editeUser.email=value.target.value
                               setEditeUser(editeUser)
                           }}/>
                </Form.Item>
            </Form>
        </Modal>
    </>)
}
export default UserPage