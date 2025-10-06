import Item from "../components/Item"
export default function Library({items}){
	console.log(items)

	return (<div className=" flex">
		{items.map(e=>{
			return (<Item widget={e.widget_name}/>)
			
		})}
		</div>)
}
