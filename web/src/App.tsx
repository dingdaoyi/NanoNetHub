import './App.css'
import HubLayout from "./components/HubLayout.tsx";
import {Route, Routes} from "react-router-dom";
import Product from "./pages/product/Product.tsx";
import ProductTsl from "./pages/ProductTsl.tsx";

function App() {
    return (
        <HubLayout>
            <Routes>
                <Route path={'product'} element={<Product/>}/>
                <Route path={'tsl'} element={<ProductTsl/>}/>
            </Routes>
        </HubLayout>
    )
}

export default App
