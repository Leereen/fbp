import './App.css';

import React from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Navbar from 'react-bootstrap/Navbar';
import Spinner from 'react-bootstrap/Spinner';

import {RandomRequest, PositionRequest, GreaterThanRequest} from './Requests';
import {MContext} from './Globals';


class Page extends React.Component {
    render() {
        return (
            <div id="app">
              <Navbar fixed="top">
                <Header />
              </Navbar>
              <Center />
              <Navbar fixed="bottom">
                <Footer />
              </Navbar>
            </div>
        )
    }
}

class Header extends React.Component {
    render() {
        return (
            <header className="app-header">
              <h1>Freshly Baked Primes</h1>
            </header>
        );
    }
}

class Footer extends React.Component {
    render() {
        let string = "The sources of this project are hosted by github "
        return (
            <footer className="app-footer">
              { string }
              <a target="_" href="https://github.com/Leereen/fbp" className="app-link">here</a>.
            </footer>
        )
    }
}

class PrimeProvider extends React.Component {
    constructor(props) {
        super(props);
        this.state = {prime: null};
    }

    render() {
        return (
            <MContext.Provider value={
                                   {
                                       state: this.state,
                                       setPrime: (value) => this.setState({prime: value})
                                   }
              }>
              { this.props.children }
            </MContext.Provider>
        )
    }
}

class Center extends React.Component {
    render() {
        return (
            <PrimeProvider>
              <PrimeResult />
              <Navbar>
                <Container fluid>
                  <Row>
                    <Col md={4}> <RandomRequest /> </Col>
                    <Col md={4}> <PositionRequest /> </Col>
                    <Col md={4}> <GreaterThanRequest /> </Col>
                  </Row>
                </Container>
              </Navbar>
            </PrimeProvider>
        )
    }
}

class PrimeResult extends React.Component {
    first = true
    render() {
        return (
            <MContext.Consumer>
              { (context) => {
                  if (this.first) {
                      this.first = false;
                      return <p>No result for now</p>;
                  } else if (context.state.prime === null) {
                      return (
                          <React.Fragment>
                            <Spinner animation="border" />
                            <span>Just a moment, lemme cook this neat Prime...</span>
                          </React.Fragment>
                      )
                  } else {
                      let op = (context.state.prime.position === 1) ? "st" :
                          (context.state.prime.position === 2) ? "nd" :
                          (context.state.prime.position === 3) ? "rd" : "th";
                      let position = "FWIW, this fine lad is the " + context.state.prime.position + op + " prime.";
                      let duration = "";
                      if (!context.state.prime.new) {
                          duration += "This one was already baked, no wonder it came out so fast. " +
                              "Just try another one!"
                      } else if (context.state.prime.duration < 10) {
                          duration += "Spent " + Number(context.state.prime.duration).toFixed(3) +
                              "s in the oven. Nice and warm, just for you.";
                      } else {
                          duration += "Wow, this one's a special! " +
                              Number(context.state.prime.duration).toFixed(3) +
                              "s to cook, long and tidy, the way I like it. It's gonna be good.";
                      }
                      return (
                          <React.Fragment>
                            <h2>{context.state.prime.prime}</h2>
                            <ul>
                              <li>{ position }</li>
                              <li>{ duration }</li>
                            </ul>
                          </React.Fragment>
                      );
                  }
              }}
            </MContext.Consumer>
        )
    }
}


function App() {
    return (
        <Page/>
    );
}


export default App;
