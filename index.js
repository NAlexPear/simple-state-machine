class Store {
  constructor(initialState){
    this.state = initialState
    this.subscribers = []
  }

  async dispatch(actionCreator){
    const action = await actionCreator()
    const newState = action(this.state)

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

class Component {
  constructor(
    element = 'div',
    template = () => '',
    attributes = [],
    events = []
  ){
    this.element = element
    this.template = template
    this.attributes = attributes
    this.events = events
  }

  createNode(state){
    const node = document.createElement(this.element)

    this.attributes.forEach(
      attribute => node.setAttribute(...Object.entries(attribute))
    )

    this.events.forEach(
      event => node.addEventListener(...Object.entries(event))
    )

    node.textContent = this.template(state)

    return node
  }
}

const incrementCount = async () => state = ({ ...state, count: state.cout + 1 })
const decrementCount = async () => {
  const reducer = await new Promise(resolve => setTimeout(
    () => resolve(state => ({ ...state, count: state.count - 1 })),
    3000
  ))

  return reducer
}

const ROOT = document.getElementById('root')
const store = new Store({ count: 0 })

const title = new Component(
  'h1',
  () => 'Click the buttons below!',
)
const incrementButton = new Component(
  'button',
  () => '+',
  [{ type: 'button' }],
  [{ click: incrementCount}],
)
const decrementButton = new Component(
  'button',
  () => '-',
  [{ type: 'button' }],
  [{ click: decrementCount }]
)
const counter = new Component(
  'p',
  state => `Count: ${ state.count }`,
)

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
        const reducer = await new Promise(resolve => setTimeout(
          () => resolve(state => ({ ...state, count: state.count - 1 })),
          3000
        ))

        return reducer
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
