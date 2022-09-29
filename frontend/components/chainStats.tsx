const ChainStats = () => {
    return (
        <div className="mt-1 px-2">
            <div className="relative">
                <div className="relative mx-auto w-10/12">
                    <div className="mx-auto max-full">
                        <dl className="rounded-lg bg-gray-800 shadow-lg sm:grid sm:grid-cols-3">
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">HEIGHT</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">1604920</dd>
                            </div>
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">TOTAL SUPPLY</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">24/7</dd>
                            </div>
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">BONDED</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">100k</dd>
                            </div>
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">INFLATION</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">100k</dd>
                            </div>
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">COMMUNITY POOL</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">100k</dd>
                            </div>
                            <div className="flex flex-col px-8 py-5">
                                <dt className="order-1 mt-2 text-lg font-medium leading-6 text-gray-400">ACTIVE VALIDATORS</dt>
                                <dd className="order-2 text-2xl font-bold tracking-tight">100k</dd>
                            </div>
                        </dl>
                   </div>
                </div>
            </div>
        </div>
    )
}

export default ChainStats