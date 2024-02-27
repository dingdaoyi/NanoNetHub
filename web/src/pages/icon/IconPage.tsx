import {ColumnsType} from "antd/es/table";
import {Button, Form, Image, Input, message, Modal, Space, Table, Upload} from "antd";
import {addIcon, deleteIcons, getIconList, Icon, updateIcon} from "@/api/iconApi.ts";
import {useEffect, useState} from "react";
import {DeleteOutlined, LoadingOutlined, PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {UploadChangeParam, UploadFile} from "antd/es/upload/interface";


function IconPage() {
    const [listData, setListData] = useState<Icon[]>([])
    const [loading, setLoading] = useState(false)
    const [icon_name, setIcon_name] = useState("")
    const [editeIcon, setEditeIcon] = useState<Icon>()
    const [editModalVisible, setEditModalVisible] = useState(false)
    const [iconUrl, setIconUrl] = useState<string>()

    function loadData() {
        setLoading(true)
        getIconList(icon_name).then((res) => {
            setListData(res)
            setLoading(false)
        })
    }

    const uploadButton = (
        <button style={{border: 0, background: 'none'}} type="button">
            {loading ? <LoadingOutlined/> : <PlusOutlined/>}
            <div style={{marginTop: 8}}>上传</div>
        </button>
    );
    useEffect(() => {
        loadData();
    }, []);

    function deleteIcon(id: number) {
        Modal.confirm({
            title: '删除图标',
            content: '确定删除该图标吗？',
            okText: '确认',
            cancelText: '取消',
            onOk: async () => {
                deleteIcons(id)
                    .then(() => {
                        handleSearch()
                    })
            },
        });

    }

    const columns: ColumnsType<Icon> = [
        {
            title: '图标名称',
            dataIndex: 'name',
            key: 'name',
            width: "150px",
        },
        {
            title: '图标',
            dataIndex: 'icon',
            key: 'icon',
            render: (text) => (
                <>
                    <Image src={text} width={100} height={100}/>
                </>
            ),
        },
        {
            title: '是否为系统图标',
            dataIndex: 'default_icon',
            key: 'default_icon',
            render: (text) => (
                <>
                    {text ? "是" : "否"}
                </>
            ),
        },
        {
            title: '操作',
            key: 'action',
            render: (_, record) => (
                (!record.default_icon) && (<Space size="middle">
                    <Button type="primary" onClick={() => {
                        setEditeIcon(record)
                        setIconUrl(record.icon)
                        setEditModalVisible(true)
                    }}>编辑</Button>
                    <Button type="primary" icon={<DeleteOutlined/>} onClick={() => deleteIcon(record.id!)}>删除</Button>
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

        if (editeIcon?.id) {
            editeIcon!.icon = iconUrl!
            updateIcon(editeIcon).then(() => {
                setLoading(false)
                setEditModalVisible(false)
                loadData()
            })
            return
        }
        if (iconUrl == undefined) {
            message.warning("请上传图标")
            return
        }
        if (editeIcon?.name == undefined) {
            message.warning("请输入图标名称")
            return
        }
        const icon = {
            name: editeIcon?.name,
            icon: iconUrl
        }
        setLoading(false)
        addIcon(icon).then(() => {
            setLoading(false)
            setEditModalVisible(false)
            loadData()
        })
    }

    function uploadIcon(info: UploadChangeParam<UploadFile>) {
        if (info.file.status !== 'uploading') {
            console.log(info.file, info.fileList);
        }
        if (info.file.status === 'done') {
            const data = info.file.response;
            setIconUrl(`/api${data.data}`)
        } else if (info.file.status === 'error') {
            console.error(info.file.response)
        }
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
                    placeholder="图标名称"
                    value={icon_name}
                    width="200px"
                    onChange={(e) => setIcon_name(e.target.value)}
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
            title={editeIcon?.id ? "编辑图标" : "新增图标"}
            open={editModalVisible}
            destroyOnClose={true}
            onCancel={() => setEditModalVisible(false)}
            onOk={async () => {
                await handleEditeSubmit();
            }}
        >
            {/* 这里可以放置编辑表单 */}
            <Form>
                <Form.Item label="图标名称">
                    <Input name="name" defaultValue={editeIcon?.name}
                           onChange={(value) => {
                               const icon = editeIcon || {
                                   name: "",
                                   icon: "",
                               }
                               icon.name = value.target.value
                               setEditeIcon(icon)
                           }}/>
                </Form.Item>
                <Form.Item label="图标">
                    <Upload
                        name="file"
                        listType="picture-card"
                        className="avatar-uploader"
                        showUploadList={false}
                        action={"/api/file/upload"}
                        onChange={uploadIcon}
                    >
                        {iconUrl ? <img src={iconUrl} alt="avatar" style={{width: '100%'}}/> : uploadButton}
                    </Upload>
                </Form.Item>
            </Form>
        </Modal>
    </>)
}

export default IconPage