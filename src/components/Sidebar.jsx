
export default function Sidebar({changeMenu}){
	return(
	<div className="flex-col flex p-2 justify-between h-full w-max">
		<div className="rounded-2xl bg-gray-950 flex flex-col w-46 h-max p-2">
			<div className="cursor-pointer select-none hover:bg-gray-900 rounded-xl w-full text-white m-auto text-center p-2 m-2 text-xl" onClick={()=>changeMenu("presets")}>Presets</div>
			<div className="cursor-pointer select-none hover:bg-gray-900 rounded-xl w-full text-white m-auto text-center p-2 m-2 text-xl" onClick={()=>changeMenu("widgets")}>Widgets</div>
			<div className="cursor-pointer select-none hover:bg-gray-900 rounded-xl w-full text-white m-auto text-center p-2 m-2 text-xl">Community</div>
		</div>	
				<div className="rounded-2xl bg-gray-950 flex flex-col w-46 h-max py-1 px-2 mb-2">
			<div className="hover:bg-gray-900 rounded-xl w-full cursor-pointer select-none text-white m-auto text-center p-2 m-2 text-xl">Settings</div>
		</div>	
	</div>


	)
}
