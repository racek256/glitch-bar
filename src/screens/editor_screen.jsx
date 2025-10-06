
import 'react-grid-layout/css/styles.css';
import 'react-resizable/css/styles.css';
import GridLayout from "react-grid-layout";
import Widget from '../components/widget';
import { useState, useRef, useEffect } from "react";
export default function EditorScreen({widgetsarr}) {
	   const ref = useRef(null);
  const [width, setWidth] = useState(0);
  const [rowHeight, setRowHeight] = useState(30);
  const totalRows = 6; // ← fixed number of rows

  useEffect(() => {
    function update() {
      if (ref.current) {
        const w = ref.current.offsetWidth;
        const h = ref.current.offsetHeight;
        setWidth(w);
        setRowHeight(h / totalRows); // adjust to always fit N rows
      }
    }
    update();
    window.addEventListener("resize", update);
    return () => window.removeEventListener("resize", update);
  }, []); 

  return (
	 <div className="m-5 w-full">
  <div ref={ref} className="relative aspect-[2/1] bg-gray-950 overflow-hidden rounded-lg shadow-lg">
    <GridLayout
      autoSize={false}
      compactType={null}
      className="absolute inset-0 h-full "
      cols={16}
      rowHeight={rowHeight/2}
      width={width}
      isResizable
      isBounded={true}
      preventCollision={true}
      resizeHandles={['se']}
      onLayoutChange={(e)=>{console.log(e)}}
    >
      {widgetsarr.map((e,i)=>(
        <div key={i} data-grid={{x:i,y:1,w:1,h:2}}>
          <Widget filename={e.widget_name}/>
        </div>
      ))}
    </GridLayout>
  </div>
</div> 
  );
}

