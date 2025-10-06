import { use, useState } from "react"
export default function Item({widget}){
	const [hovered, setHovered] = useState(false)
	return (
	<div className="relative w-56 h-37 bg-gray-900 shadow-lg rounded-2xl flex m-2 " 
		onMouseEnter={()=> setHovered(true)} 
		onMouseLeave={()=>setHovered(false)}>
		<iframe className=" w-full h-full" src={`http://localhost:3030/widgets/${widget}`}/>
		{hovered ? <div className="absolute bg-gray-900 top-0 left-0 h-full w-full rounded-xl opacity-50 flex flex-col justify-center">
			<button className="!bg-gray-900 !text-white !text-xl">Add to preset</button>
			<button className="!bg-gray-900 !text-white !text-xl">Edit widget</button>
		
			</div> : <div/>}
			
	</div>
	)
}
