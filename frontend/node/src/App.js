import React, { Component } from 'react';
import './App.css';

class App extends Component {

  constructor(props) {
    super(props);

    this.state = {
      log: [],
      sendText: ''
    };

    this.handlePayload = this.handlePayload.bind(this);
    this.updateSendText = this.updateSendText.bind(this);
    this.handleSend = this.handleSend.bind(this);

    this.logRef = React.createRef();
  }

  componentDidMount() {
    this.webSocket = new WebSocket('ws://localhost:9000/ws');

    this.webSocket.onopen = (event) => {
      console.log('connected to websocket');
    };

    this.webSocket.onmessage = (event) => {
      const payload = JSON.parse(event.data);
      
      this.handlePayload(payload);
    };

    this.webSocket.onclose = (event) => {
      console.log('disconnected from websocket');
    };

    setInterval(() => {
      this.handlePayload({
        type: 'event',
        message: 'a rock fell down'
      })
    }, 3000);
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.state.log.length > prevState.log.length) {
      const logElem = this.logRef.current;
      logElem.scrollTop = logElem.scrollHeight - logElem.clientHeight;
    }
  }

  updateSendText(event) {
    this.setState({
      sendText: event.target.value
    });
  }

  handlePayload(payload) {
    // payload is json object returned from your ws api
    console.log(`received: ${payload.type} - ${payload.message}`);

    this.setState({
      log: this.state.log.concat([payload.message])
    });
  }

  handleSend() {
    const sendPayload = {
      message: this.state.sendText
    };

    this.webSocket.send(JSON.stringify(sendPayload));

    this.setState({
      sendText: ''
    });
  }

  render() {
    const logMessages = this.state.log.map((event, i) => {
      return (
        <div key={i}>
          {event}
        </div>
      )
    });

    return (
      <div className="App">
        <div ref={this.logRef} className="log">
          {logMessages}
        </div>
        <input type="text" className="send-textbox" value={this.state.sendText} onChange={this.updateSendText} />
        <button onClick={this.handleSend}>Send</button>
      </div>
    );
  }
}

export default App;
