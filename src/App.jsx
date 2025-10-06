import { useState, useEffect, use } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import EditorScreen from "./screens/editor_screen";
import Sidebar from "./components/Sidebar";
import Navbar from "./components/Navbar";
import Library from "./screens/Library";
function App() {
	let [Menu, changeMenu] = useState("Main");
	let [widgets, setWidgets] = useState(["Loading widgets"]);
	let [presets, setPresets] = useState(["loading presets"])
	useEffect(()=>{
		async function loadwidgets(){
			console.log("fetching widgets");
			const data = await fetch("http://localhost:3030")
			let response = await data.json();
			console.log(response)
			setWidgets(response)
		}
		async function loadpresets(){
			console.log("fetching widgets");
			const data = await fetch("http://localhost:3030/presets")
			let response = await data.json();
			console.log(response)
			setPresets(response)
		}
		loadwidgets()
		loadpresets()
	},[])


  return (
    <main className="w-screen h-screen p-1 bg-[#141419] overflow-hidden">
	<div className="flex flex-col h-screen overflow-hidden">
	<Navbar/>
	  <div className="flex h-full">
	<Sidebar changeMenu={changeMenu}/> 	
	  {Menu == "Main" ? (
	<Library items={widgets}/>
	  ) : (<EditorScreen widgetsarr={widgets}/>)}
	  </div>
	  </div>
	
    </main>
  );
}

export default App;
// <EditorScreen widgetsarr={widgets} /> 
