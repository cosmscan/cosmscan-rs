import { BASE_URL, Block } from "./types";

type ApiListBlockProps = {
    chainId: number;
    limit: number;
    offset: number;
};

const apiListBlock = ({ chainId, limit, offset }: ApiListBlockProps): Promise<Block[]> => {
    return fetch(`${BASE_URL}/api/block/list/${chainId}?limit=${limit}&offset=${offset}`)
        .then((res) => res.json())
};

const apiGetBlock = (chainId: number, height: number): Promise<Block> => {
    return fetch(`${BASE_URL}/api/block/${chainId}/${height}`)
        .then((res) => res.json())
}

export {
    apiListBlock,
    apiGetBlock,
}