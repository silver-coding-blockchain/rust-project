const Marketplace=artifacts.require('./Marketplace.sol')
contract('Marketplace',(accounts)=>{
    let Marketplace
    before(async()=>{
        Marketplace=await Marketplace.deployed()
    })
})