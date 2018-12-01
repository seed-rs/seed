import * as React from 'react'
import * as ReactDOM from 'react-dom'

interface MainProps {}

interface MainState {
    val: number,

}

const MyComponent = ({items, add_}: {items: number[], add_: Function}) => (
    <div style={{display: 'flex', margin: 'auto'}}>
        <h2>Items:</h2>
        {items.map(item => (<h4 
            key={item}  // Required by React for internal use
            onClick={() => add_(item)}  // Callback/interaction example
        >
            item.toString()
        </h4>))}
    </div>
)

class Main extends React.Component<MainProps, MainState> {
    constructor(props) {
        // constructor is mainly OOP boilerplate
        super(props)
        this.state = {  // Initial values for state
            items: [1, 2, 3]  
        }

        this.add = this.add.bind(this)
        this.save = this.save.bind(this)
    }

    add(val: number) {
        this.setState({items: [...this.state.items, val]})
    }

    save() {
        // Demo for calling a server
        fetch('https://test.com', {
            method: 'POST', 
            body: JSON.stringify(this.state.items)
        // Async promise behavior native to JS
        }).then(
            result => console.log("Saved!", result.json())
            )
    }

    render() {
        return (
            <MyComponent items={this.state.items} add={this.state.add}/>
        )
    }
}

ReactDOM.render(<Main />, document.getElementById('react'))