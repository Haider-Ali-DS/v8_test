import {Connection as $im0Rw$Connection, clusterApiUrl as $im0Rw$clusterApiUrl} from "@solana/web3.js";


(async ()=>{
    let con = new (0, $im0Rw$Connection)((0, $im0Rw$clusterApiUrl)("devnet"));
    console.log(await con.getBlockHeight());
})();


//# sourceMappingURL=indexm.js.map
