import { CalendarIcon, MapPinIcon, UsersIcon } from '@heroicons/react/20/solid'

const transactions = [
  {
    id: 1,
    height: '800',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 2,
    height: '799',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 3,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 4,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 5,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 6,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 7,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
  {
    id: 8,
    height: '798',
    txHash: 'AB30D603F7F37D7E530911C27853B9F4EBAFFF6FCCCDE98583786C047B0F05C6',
    txType: 'MsgSend',
    txes: 100,
    createdAt: "7 seconds ago",
  },
]

const TransactionSummaryList = () => {
  return (
    <div className="overflow-hidden rounded-md">
      <ul role="list" className="divide-y divide-gray-600">
        {transactions.map((block) => (
          <li key={block.id}>
            <a href="#" className="block hover:bg-slate-900">
              <div className="px-4 py-4 sm:px-6">
                <div className="flex items-center justify-between">
                  <p className="truncate text-md text-blue-400">{block.txHash.slice(0, 20)}</p>
                  <div className="ml-2 flex flex-shrink-0">
                    <p className="inline-flex rounded-full bg-blue-100 px-2 text-xs font-semibold leading-5 text-blue-800">
                      {block.txType}
                    </p>
                  </div>
                </div>
                <div className="mt-2 sm:flex sm:justify-between">
                  <div className="sm:flex">
                    <div>
                        <span className="text-gray-500">Height: </span>
                        <span>{block.height}</span>
                    </div>
                  </div>
                  <div className="mt-2 flex items-center text-gray-500 sm:mt-0">
                    <CalendarIcon className="mr-1.5 h-5 w-5 flex-shrink-0 text-gray-400" aria-hidden="true" />
                    <p>{block.createdAt}</p>
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

export default TransactionSummaryList;