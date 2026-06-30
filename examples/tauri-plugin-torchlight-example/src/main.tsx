import "./style.css"

import { useEffect, useState } from "react"
import { createRoot } from "react-dom/client"
import {
    isAvailable,
    isEnabled,
    onTorchModeChanged,
    setTorch,
} from "tauri-plugin-torchlight-api"

function App() {
    const [available, setAvailable] = useState<boolean | null>(null)
    const [on, setOn] = useState(false)
    const [level, setLevel] = useState(1)
    const [error, setError] = useState("")

    useEffect(() => {
        isAvailable().then(setAvailable).catch(() => setAvailable(false))
        isEnabled().then(setOn).catch(() => {})

        const listenerPromise = onTorchModeChanged(({ enabled }) => setOn(enabled))
        return () => {
            listenerPromise.then((listener) => listener.unregister()).catch(() => {})
        }
    }, [])

    const toggle = async () => {
        const next = !on
        try {
            await setTorch(next, next ? level : undefined)
            setOn(next)
            setError("")
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e))
        }
    }

    return (
        <div className="container">
            <div className="title">TorchLight</div>
            <button
                className="button"
                id="torchButton"
                disabled={available === false}
                onClick={toggle}
            >
                {on ? "Turn Off" : "Turn On"}
            </button>
            <label className="level">
                Brightness: {level.toFixed(2)}
                <input
                    type="range"
                    min={0}
                    max={1}
                    step={0.05}
                    value={level}
                    onChange={(e) => setLevel(Number(e.target.value))}
                />
            </label>
            {available === false && (
                <div className="error">Torch is not available on this device.</div>
            )}
            <div className="error" id="error">
                {error}
            </div>
        </div>
    )
}

const root = document.getElementById("root")
if (root) {
    createRoot(root).render(<App />)
}
