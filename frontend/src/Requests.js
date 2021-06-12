import React from 'react';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';

import {HOST, MContext} from './Globals';


class PrimeRequest extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            value: null
        };
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleSubmit(event, context) {
        event.preventDefault();
        let URL = this.getRoutePrefix();
        if ( this.state.value != null) {
            URL += this.state.value;
        }
        context.setPrime(null);
        fetch(URL)
            .then(res => res.json())
            .then(
                (result) => { context.setPrime(result); },
                (error) => {}
            );
    }
}

class RandomRequest extends PrimeRequest {
    getRoutePrefix() {
        return HOST;
    }

    render() {
        return (
            <div className="getPrime">
              <MContext.Consumer>
                { (context) => (
                    <Form onSubmit={ (e) => this.handleSubmit(e, context) }>
                      <Form.Group controlId="request-form">
                        <Button variant="primary" type="submit">
                          Get your own prime!
                        </Button>
                      </Form.Group>
                    </Form>
                )}
              </MContext.Consumer>
            </div>
        );
    }
}

class PositionRequest extends PrimeRequest {
    constructor(props) {
        super(props);
        this.state = { value: 0 };
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        this.setState({value: event.target.value});
    }

    getRoutePrefix() {
        return HOST + "/at_position/";
    }

    render() {
        return (
            <div className="getPrime">
              <MContext.Consumer>
                { (context) => (
                    <Form onSubmit={ (e) => this.handleSubmit(e, context) }>
                      <Button variant="primary" type="submit">
                        Get your own prime!
                      </Button>
                      <Form.Group >
                        <Form.Label>at position...</Form.Label>
                        <Form.Control type="number" min="0"
                                      placeholder="Enter value"
                                      onChange={ this.handleChange } />
                      </Form.Group>
                    </Form>
                )}
              </MContext.Consumer>
            </div>
        );
    }
}

class GreaterThanRequest extends PrimeRequest {
    constructor(props) {
        super(props);
        this.state = {
            value: 0,
            strict: false
        };

        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        this.setState({value: event.target.value});
    }

    check(event) {
        this.setState({strict: !this.state.strict});
    }

    getRoutePrefix() {
        if (this.state.strict) {
            return HOST + "/first_greater_than//";
        } else {
            return HOST + "/greater_than/";
        }
    }

    render() {
        return (
            <div className="getPrime">
              <MContext.Consumer>
                { (context) => (
                    <Form onSubmit={ (e) => this.handleSubmit(e, context) }>
                      <Button variant="primary" type="submit">
                        Get your own prime!
                      </Button>
                      <Form.Group>
                        <Form.Label>greater than...</Form.Label>
                        <Form.Control type="number" min="0"
                                      placeholder="Enter value"
                                      onChange={ this.handleChange } />
                      </Form.Group>
                      <Form.Group>
                        <Form.Check type="checkbox"
                                    label="First greater than..."
                                    onChange={ (e) => this.check(e) }/>
                      </Form.Group>
                    </Form>
                )}
              </MContext.Consumer>
            </div>
        );
    }
}

export {RandomRequest, PositionRequest, GreaterThanRequest};
