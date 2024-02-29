import {Button, Checkbox, Form, Input, Typography} from 'antd';
import logo from "@/assets/react.svg"
import {loginApi} from "@/api/userAip.ts"
import {saveToken} from "@/store/userInfo.tsx";
import {useNavigate} from "react-router-dom";
import {LoginType} from "@/types/api.ts";

const {Title} = Typography;
const LoginContent = () => {

    const navigate = useNavigate();
    const onFinish = async (values: object) => {
        console.log("eee")
        const token = await loginApi(values);
        saveToken(token)
        navigate('/')
    };

    return (
        <div className={"flex justify-center h-screen items-center  bg-gray-200 "}>
            <div className={"shadow-md p-6 text-center max-w-96 w-full rounded-3xl  bg-gray-50"}>
                <Form
                    name="basic"
                    initialValues={{remember: true}}
                    onFinish={onFinish}
                    layout={"vertical"}
                    autoComplete="off"
                >
                    <div className={"flex  justify-center mb-4"}>
                        <img src={logo} alt="图片" style={{
                            width: 50,
                            height: 50,
                        }}/>
                    </div>
                    <Title level={3}>物联网平台</Title>
                    <Form.Item<LoginType>
                        label="用户名"
                        name="username"
                        rules={[{required: true, message: '请输入用户名'}]}
                    >
                        <Input/>
                    </Form.Item>

                    <Form.Item<LoginType>
                        label="密码"
                        name="password"
                        rules={[{required: true, message: '请输入密码'}]}
                    >
                        <Input.Password/>
                    </Form.Item>

                    <Form.Item<LoginType>
                        name="remember"
                        valuePropName="checked"
                        className={"flex self-start"}
                    >
                        <Checkbox>记住登录</Checkbox>
                    </Form.Item>

                    <Form.Item>
                        <Button className={"w-full"} type={"primary"} htmlType={"submit"}>提交</Button>
                    </Form.Item>
                </Form>

            </div>
        </div>
    )

}


export default LoginContent;
