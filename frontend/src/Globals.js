import React from 'react';

//var HOST = "https://freshlybakedprimes.eu/api/";
export const HOST = "http://localhost:8010/proxy/api/";
export const MContext = React.createContext();

export function numberWithSpaces(x) {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, " ");
}
