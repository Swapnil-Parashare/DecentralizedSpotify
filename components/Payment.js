import { useEffect, useState} from 'react'
import { getProgramInstance } from '../utils/utils'                     // Our Smart Contract Instace.
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { SOLANA_HOST } from '../utils/const'                            // It holds our Endpoint. (Quick Node)
import { useWallet } from '@solana/wallet-adapter-react'                // It holds the instance of our connected phantom wallet.
import { PublicKey } from '@solana/web3.js'

import Homepage from '../pages/homepage'

const anchor = require('@project-serum/anchor');                        // Here we are getting the entire github anchor project.

const { web3 } = anchor;                                                // Now we are destructing the anchor, and getting what we exactly want from it.

const {SystemProgram} = web3;                                           // Destructing web3.

const utf8 = anchor.utils.bytes.utf8;

const defaultAccounts = {
  tokenProgram : TOKEN_PROGRAM_ID,                                      // Contract Address.
  clock : anchor.web3.SYSVAR_CLOCK_PUBKEY,
  systemProgram : SystemProgram.programId
}

const styles = {
  main: `w-screen h-screen bg-white text-black flex flex-col justify-center items-center`,
  button: `bg-[#22C55E] m-3 text-white font-bold py-4 px-7 rounded-full hover:opacity-70 transition`,
  text: `text-5xl text-black mb-10`,
  buttons: `flex items-center`,
}



const Payment = () => {

  const wallet = useWallet();                                            // Instance of connected wallet.
  const connection = new anchor.web3.Connection(SOLANA_HOST);            // Connection is created using 'Endpoint' (QuickNode)
  const program = getProgramInstance(connection,wallet);                 // Instance of our smart contract.

  const [payers, setPayers] = useState([]);
  const [isPaid, setPaid] = useState(false);




 useEffect(
  
  () => {                                                     // This function will run as soon as Payment component is loaded.
    if(wallet.connected)
    {
       getAllWallets();                                       // So as soon as the component loads, we are checking is our current wallet is present in payed accounts list.
    }
  },

  [wallet.connected,                                             // If our connected wallet is changed or disconnected, at each such instance this function will be executed along with initial loading of component.
   isPaid]                                                       // If isPaid state is changed then also it will re-run.
  )




  const getAllWallets = async () => {

    const payerlist = await program.account.payerAccount.all();                // Smart Contract will get a list of all the accounts which have payed the fees.

    setPayers(payerlist);                                                      // We will update our useState (payers) using setPayers with the recieved list.

    payerlist.forEach(                                                         // We will iterate over the list and check if current wallet is present in that list
      payer => {
        if(payer.account.wallet.toBase58() == wallet.publicKey.toBase58())     // If current wallet is present in payed accounts list, Then set another useState i.e isPaid as true.
        {
          setPaid(true);
        }
      }
    )
  }


  const payClicked = async () => {

    let [payerSigner] = await anchor.web3.PublicKey.findProgramAddress
    (
      [utf8.encode('payer'), wallet.publicKey.toBuffer()],
      program.programId
    );

    

    let payerInfo;

    try
    {
  
      payerInfo = await program.account.payerAccount.fetch(payerSigner)
    } 
    catch (error)
    {
      try
      {

        console.log(`Current User : ${wallet.publicKey}`)
        console.log(`PaySigner : ${payerSigner}`)

        await program.rpc.acceptPayment({
          accounts : {
            payerWallet : payerSigner,                                        // Derived Address (For this addresses Solana Explorer says Account does not exist.)
            receiver : new PublicKey(
               "H4qtAFP8jVwjWrUPqzvP9MfbrAoqqFLdrYD8XWqG9fc4"                 // Reciever           
            ),
            authority : wallet.publicKey,                                     // Current User (Payer)
            ...defaultAccounts,
          },
        })

        alert("Transaction Successful!!!")
      } 
      catch (error) 
      {   
        alert( `This is Error :  ${error.message}`)                             // Here we are facing error.
      }
    }

  }


  if(isPaid) return <Homepage/>



  return (

    <div className = {styles.main} >

      <p className = {styles.text}> Make Payment</p>
      
      <div className= {styles.buttons} >

        <button 
          className = {styles.button}
          onClick = {payClicked}
        >
          Pay 0.1 Sol
        </button> 

        <button
          className = {styles.button}
          onClick = {getAllWallets}
        >
          Verify Payment
        </button>
  
      </div>
    
    </div>
  )
}

export default Payment


/*
Current User : 9bmPLUBoxeBE8ognku1zFNwDh53HA8u1jkpeGjzqcA5
Payer Signer : BAzUpCkbfYU6HkEweSJZGkCthNzT7ebLu2Q8CJizeXNo
*/