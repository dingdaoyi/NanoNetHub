import {Button, Checkbox, Form, Input} from 'antd';
import './login.less'
import logo from "../../assets/react.svg"
import {loginApi, LoginType} from "../../api/userAip.ts"
import {saveUser} from "../../store/userInfo.tsx";
import {useNavigate} from "react-router-dom";


const LoginContent = () => {

    let navigate = useNavigate();
    const onFinish = async (values: any) => {

        let userinfo = await loginApi(values);
        saveUser(userinfo)
        navigate('/')
    };

    return (
        <div className={"login"}>
            <Form
                name="basic"
                className={"login-form"}
                initialValues={{remember: true}}
                onFinish={onFinish}
                layout={"vertical"}
                autoComplete="off"
            >
                <img src={logo} alt="图片" className={logo}/>
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
                >
                    <Checkbox>记住登录</Checkbox>
                </Form.Item>

                <Form.Item>
                    <Button type="primary" htmlType="submit">
                        登录
                    </Button>
                </Form.Item>
            </Form>
        </div>
    )
}


export default LoginContent;
