import { BASE_URL, Block, Transaction } from "./types";


const apiListTxInBlock = (chainId: number, height: number): Promise<Transaction[]> => {
    return fetch(`${BASE_URL}/api/tx/list/${chainId}/${height}`)
        .then((res) => res.json())
};

export {
    apiListTxInBlock,
}