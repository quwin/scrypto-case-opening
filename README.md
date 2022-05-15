# scrypto-case-opening : A CS:GO-based randomized case opening

This blueprint implements weapon skins from Counter-Strike: Global Offensive as NFTs. In the blueprint, users can exchange XRD for Keys, use Keys to open a case (I chose the Chroma 2 Case to implement), and trade-up skins to create a higher-tier skin. 

Some current limitations: 
1. Not all knives are implmented (There are a few hundred knive skins which would clutter the blueprint lol)
2. Cases do not have to be bought and burnt (This could be done easily, but I was too lazy to add it)
3. Nametags are not implemented (Same thing)
4. Float minimums/maximums of individual skins do not have data (The system is in place for float min/max, but I don't wanna get the data)
5. Different cases would have to be coded as seperate HashMaps, and a new open_case method would need to be added for that new case.
6. Trade-ups with skins of different cases mixed together is not implemented.

# How to use:
1. Reset the simulator
```
resim reset
```
2. Create a new account to administer the component. Save the account address to `$a`. Additionally, save the radix token address as `$xrd`
```
resim new-account
xrd=030000000000000000000000000000000000000000000000000004
```
3. Publish the package. Save the package address to `$p`
```
resim publish .
```
4. Instantiate a Case component. 
  Save the necessary ResourceAddresses for later 
  The first two ResourceAddresses generated are for the developer badge (to control system badges)
  and the system badge (to authorize changing, minting, and burning all NFTs)
```
resim call-function $p Case new
skin=[Third Resource Address generated]
key=[Last ResourceAddress generated]
```
5. Buy keys to open cases. Any number of keys can be bought at once, provided you have enough XRD. Each Key costs 25 XRD (since CS:GO Keys are $2.50)
```
resim call-method $c buy_keys 1,$xrd 25
```
6. Open a case. The weapon you get will be randomized according to the odds. On average, it'll take a few hundred cases to get a knife (Yeah the odds suck in CS:GO) Check to see what skin you got afterwards
```
resim call-method $c open_case 1,$key
resim show $a
```
7. Keep rolling! Try to see the coolest or lowest Float skin you can get!

8. Trade-ups! You can trade ten skins of the same rarity for another skin of a higher rarity. 

10x Mil-Specs -> Restricted, 10x Restricted -> Classified, and 10x Classified -> Covert
```
resim call-method $c trade_up "#$[NFTID1],#$[NFTID2],#$[NFTID3],#$[NFTID4],#$[NFTID5],#$[NFTID6],#$[NFTID7],#$[NFTID8],#$[NFTID9],#$[NFTID10],$skin"
```

The resulting float of the skin will be the average of all floats used for trading-up, so use lower floats in trade ups for better resulting skin floats.
