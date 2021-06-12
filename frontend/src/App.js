import './App.css';

import React from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Navbar from 'react-bootstrap/Navbar';
import Spinner from 'react-bootstrap/Spinner';
import Jumbotron from 'react-bootstrap/Jumbotron';

import {RandomRequest, PositionRequest, GreaterThanRequest} from './Requests';
import {MContext, numberWithSpaces} from './Globals';


class Page extends React.Component {
    render() {
        return (
            <div id="app">

              <Navbar static="top" className="app-header">
                <Header />
              </Navbar>

              <Container fluid className="flex-grow-1 app-main">
                <Center />
              </Container>

              <Footer />

            </div>
        );
    }
}

class Header extends React.Component {
    render() {
        return (
            <header>
              <h1 className="app-header-title">
                Freshly Baked Primes
              </h1>
            </header>
        );
    }
}

class Footer extends React.Component {
    render() {
        let string = "The sources of this project are hosted by github ";
        return (
            <footer className="app-footer">
              <p>
                This page and all of its content should not be taken too seriously.<br/>
                In particular, do not use these Primes in your crypto shenanigans. They're good, but not quite battle-hardened yet.<br/><br/>
                { string } <a target="_" href="https://github.com/Leereen/fbp" className="app-link">here</a>.
              </p>
            </footer>
        );
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
        );
    }
}

class Center extends React.Component {
    render() {
        return (
            <PrimeProvider>
              <Jumbotron className="app-main-result">
                <PrimeResult />
              </Jumbotron>
              <Navbar className="justify-content-center" fixed="container">
                <Row>
                  <Col md={4}> <RandomRequest /> </Col>
                  <Col md={4}> <PositionRequest /> </Col>
                  <Col md={4}> <GreaterThanRequest /> </Col>
                </Row>
              </Navbar>
            </PrimeProvider>
        );
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
                      return (
                          <React.Fragment>
                            <p className="app-main-result-text">No result yet</p>
                            <p className="app-main-comment-text">Use the underneath buttons</p>
                          </React.Fragment>
                      );
                  } else if (context.state.prime === null) {
                      return (
                          <React.Fragment>
                            <Spinner animation="border" />
                            <br/>
                            <span>Just a moment, lemme cook this neat Prime...</span>
                          </React.Fragment>
                      );
                  } else {
                      let op = (context.state.prime.position === 1) ? "st" :
                          (context.state.prime.position === 2) ? "nd" :
                          (context.state.prime.position === 3) ? "rd" : "th";
                      let position = "This fine lad is the " +
                          numberWithSpaces(context.state.prime.position)
                          + op + " prime.";
                      let duration = "";
                      if (!context.state.prime.new) {
                          duration += "It was already cooked, no wonder it came out so fast " +
                              "(barely " + Number(context.state.prime.duration*1000).toFixed(3) + "ms). " +
                              "Just try another one!";
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
                            Here's your number: <h2 className="app-main-result-text">
                              {numberWithSpaces(context.state.prime.prime)}
                            </h2>
                            <p className="app-main-comment-text">
                              Uninteresting facts:
                            </p>
                            <ul>
                              <li>{ position }</li>
                              <li>{ duration }</li>
                            </ul>
                          </React.Fragment>
                      );
                  }
              }}
            </MContext.Consumer>
        );
    }
}


function App() {
    return (
        <Page/>
    );
}


export default App;
