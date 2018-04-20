class Store {
  constructor(initialState){
    this.state = initialState
    this.subscribers = []
  }

  async dispatch(actionCreator){
    const action = await actionCreator()
    const newState = action(this.state)

    console.log('click heard, new state is', newState)

    this.setState(newState)
  }

  subscribe(subscriber){
    this.subscribers.push(subscriber)
  }

  setState(newState){
    this.state = newState
    this.subscribers.forEach(subscriber => subscriber(this.state))
  }
}


const ROOT = document.getElementById('root')
const store = new Store({ count: 0 })

const view = state => ({
  html: `
    <h1>Click the buttons below!</h1>
    <button id="increment" type="button">+</button>
    <button id="decrement" type="button">-</button>
    <p>Count: ${ state.count }</p>
  `,
  events: [
    {
      event: 'click',
      selector: '#increment',
      handler: async function incrementCount(){
        return state => ({ ...state, count: state.count + 1 })
      }
    },
    {
      event: 'click',
      selector: '#decrement',
      handler: async function decrementCount(){
        return state => ({ ...state, count: state.count - 1 })
      }
    }
  ]
})

const render = ({ html, events }) => {
  ROOT.innerHTML = html

  events.forEach(
    ({ event, selector, handler }) => document
      .querySelector(selector)
      .addEventListener(event, () => store.dispatch(handler))
  )
}

const app = state => render(view(state))

app(store.state)

store.subscribe(app)
