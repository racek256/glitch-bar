import { useState } from "react";
export default function Widget({filename, display}){
	display = true
	if(display){
	return (
   		 <div className=" h-full w-full overflow-hidden scroll-none bg-blue-400 "         style={{
       		   transform: "scale(1)", 
		   transformOrigin: "top left",
       		 }} ><iframe 
		scrolling="no"  
		className="h-full w-full overflow-hidden pointer-events-none" 
		src={`http://localhost:3030/widgets/${filename}`}
		/></div>
  	);


	}else{
	return (
   		 <div className=" h-full w-full overflow-hidden scroll-none"         style={{
       		   transform: "scale(1)", 
       		   transformOrigin: "top left",
       		 }} ><iframe 
		scrolling="no"  
		className="h-full w-full overflow-hidden" 
		src={`http://localhost:3030/widgets/${filename}`}
		/></div>
  	);
}
}
