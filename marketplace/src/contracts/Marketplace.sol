pragma solidity ^0.5.0;

contract Marketplace{
    string public name;
    uint public productCount=0;
    mapping(uint=>Product) public products;

    struct Product{
        uint id;
        string name;
        uint price;
        address owner;
        bool purchased;
    }
    constructor() public{
        name="Dapp University Marketplace";
    }
}