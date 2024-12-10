"use client"

import {useCallback, useState} from "react";
import {startTransaction} from "@/app/chargers/[chargerId]/startTransaction";

export default function StartTransactionButton({chargerId, outletId}: { chargerId: string, outletId: string }) {
    const [loading, setLoading] = useState(false);
    const callback = useCallback(() => {
        setLoading(true)
        startTransaction(chargerId, outletId).finally(() => setLoading(false))
    }, [setLoading, chargerId, outletId])

    return <button type="button"
                   onClick={callback}
                   className="text-neutral-900 bg-white border border-neutral-300 focus:outline-none hover:bg-neutral-100 focus:ring-4 focus:ring-neutral-100 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-neutral-900 dark:text-white dark:border-neutral-600 dark:hover:bg-neutral-800 dark:hover:border-neutral-600 dark:focus:ring-neutral-700">{loading ? "Loading..." : "Start Transaction"}
    </button>
}