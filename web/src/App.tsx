import './App.css'
import HubLayout from "./components/HubLayout.tsx";
import {Route, Routes} from "react-router-dom";
import Product from "./pages/product/Product.tsx";
import ProductTsl from "./pages/tsl/ProductTsl.tsx";
import DeviceInfo from "./pages/device/DeviceInfo.tsx";
import DeviceDetails from "./pages/device/DeviceDetails.tsx";
import IconPage from "./pages/icon/IconPage.tsx";

function App() {

    return (
        <HubLayout>
            <Routes>
                <Route path={'product'} element={<Product/>}/>
                <Route path={'*'} element={<Product/>}/>
                <Route path={'tsl/:productId'} Component={ProductTsl}/>
                <Route path={'device'} Component={DeviceInfo}/>
                <Route path={'icon'} Component={IconPage}/>
                <Route path={'device/details/:deviceId'} Component={DeviceDetails}/>
            </Routes>
        </HubLayout>
    )
}

export default App
