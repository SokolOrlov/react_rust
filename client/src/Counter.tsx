import * as React from 'react';
import './styles.css';

export default function Counter(){
  const [count,setCount] = React.useState(0)
  
  const increment = () => { 
    setCount(count+1)
  };

  const decrement = () => {
    setCount(count-1)
  };
  return(
  <div className='base'>
    <h1>{count}</h1>
    <button onClick={increment}>Increment</button>
    <button onClick={decrement}>Decrement</button>
  </div>
  )
}


// export default class Counter extends React.Component {
//   state = {
//     count: 0
//   };

//   increment = () => {
//     console.log('increment')
//     this.setState({
//       count: (this.state.count + 1)
//     });
//   };

//   decrement = () => {
//     this.setState({
//       count: (this.state.count - 1)
//     });
//   };

//   render () {
//     return (
//       <div>
//         <h1>{this.state.count}</h1>
//         <button onClick={this.increment}>Increment</button>
//         <button onClick={this.decrement}>Decrement</button>
//       </div>
//     );
//   }
// }