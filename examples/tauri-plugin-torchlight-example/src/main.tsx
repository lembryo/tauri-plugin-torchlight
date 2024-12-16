import "./style.css"

import { invoke } from "@tauri-apps/api/core"
import { createRoot } from "react-dom/client"

const root: HTMLElement | null = document.getElementById("root")
if (root) {

    const click = () => {
        const error = document.getElementById("error") as HTMLDivElement
        const button = document.getElementById("torchButton") as HTMLButtonElement
        if (button.innerText === "Turn On") {
            button.innerText = "Turn Off"
            invoke("plugin:torchlight|torch", {
                enabled: true
            })
                .then(() => {
                    error.innerText = ""
                })
                .catch((error) => {
                    error.innerText = error.message
                })
        } else {
            button.innerText = "Turn On"
            invoke("plugin:torchlight|torch", {
                enabled: false
            })
                .then(() => {
                    error.innerText = ""
                })
                .catch((error) => {
                    error.innerText = error.message
                })
        }
    }

    createRoot(root).render(<>
        <div className="container">
            <div className="title">TorchLight</div>
            <button className="button" id="torchButton" onClick={() => {
                click()
            }}>
                Turn On
            </button>
            <div className="error" id="error">
            </div>
        </div>
    </>)
}
