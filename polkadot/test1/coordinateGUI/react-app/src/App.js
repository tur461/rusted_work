import logo from './logo.svg';
import './App.css';
import axios from 'axios';
import { useEffect, useState } from 'react';


function App() {
  const [imgSrc, setImgSrc] = useState(null);

  const fetchNewGraph = _ => {
    axios.get('http://localhost:8448/getNewGraph')
    .then(response => {
      console.log('got response');
      setImgSrc('data:image/png;base64,' + response.data.image_url);
    })
    .catch(err => {
      setImgSrc(null)
      console.log('error', err);
    });
  }

  useEffect(_ => {
    fetchNewGraph();
  }, [])
  
  return (
    <div className="App">
      <button onClick={_ => fetchNewGraph()}>Fetch New One!</button><br />
      {
        imgSrc == null ? null :
        <img src={imgSrc} alt="graph" loading='lazy'/>
      }
    </div>
  );
}

export default App;
