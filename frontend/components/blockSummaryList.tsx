import { CalendarIcon, MapPinIcon, UsersIcon } from '@heroicons/react/20/solid'
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { apiListBlock } from '../apis/blockApi'
import { shortenString } from '../utils/strings';

const BlockSummaryList = () => {
    let [chainId, setChainId] = useState(1);
    const { isLoading, isError, data, error } = useQuery(["blocks", chainId], () => apiListBlock({ chainId: chainId, limit: 10, offset: 0 }), { refetchInterval: 2500 })

    return (
        <div className="overflow-hidden rounded-md">
            <ul role="list" className="divide-y divide-gray-600">
                {data && data.map((block) => (
                <li key={block.id}>
                    <a href={`/blocks/${chainId}/${block.height}`} className="block hover:bg-slate-900">
                    <div className="px-4 py-4 sm:px-6">
                        <div className="flex items-center justify-between">
                        <p className="truncate text-md text-blue-400">{block.height}</p>
                        <div className="ml-2 flex flex-shrink-0">
                            <p className="inline-flex rounded-full bg-green-100 px-2 text-xs font-semibold leading-5 text-green-800">
                            {shortenString(block.proposer_address, 10)}
                            </p>
                        </div>
                        </div>
                        <div className="mt-2 sm:flex sm:justify-between">
                        <div className="sm:flex">
                            <p className="flex items-center text-gray-500">
                                <span>Txes: </span>
                                <span className="text-white pl-2">0</span>
                            </p>
                            <p className="flex item-center text-gray-500 md:pl-4">
                                <span>Block hash:</span>
                                <span className="text-white pl-2">{shortenString(block.block_hash, 10)}</span>
                            </p>
                        </div>
                        <div className="mt-2 flex items-center text-gray-500 sm:mt-0">
                            <CalendarIcon className="mr-1.5 h-5 w-5 flex-shrink-0 text-gray-400" aria-hidden="true" />
                            <p>{block.block_time}</p>
                        </div>
                        </div>
                    </div>
                    </a>
                </li>
                ))}
            </ul>
        </div>
    )
}

export default BlockSummaryList;