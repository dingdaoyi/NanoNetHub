import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import {HashRouter as Router, Route, Routes} from "react-router-dom";
import {ConfigProvider} from "antd";
import zhCN from "antd/locale/zh_CN"
import Login from "./pages/login/Login.tsx";

ReactDOM.createRoot(document.getElementById('root')!).render(
    <Router>
        <ConfigProvider locale={zhCN}>
            <Routes>
                <Route path="/admin/*" element={<App/>}/>
                <Route path="*" element={<App/>}/>
                <Route path="/login" element={<Login/>}/>
            </Routes>
        </ConfigProvider>
    </Router>
)
