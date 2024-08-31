import "./style.css"
import { createRoot } from "react-dom/client"
import { invoke } from "@tauri-apps/api/core"

const root: HTMLElement | null = document.getElementById("root")
if (root) {

    const click = () => {
        const button = document.getElementById("torchButton") as HTMLButtonElement
        if (button.innerText === "Turn On") {
            button.innerText = "Turn Off"
            invoke("plugin:torchlight|torch_off")
                .then((response) => {
                    console.log(response)
                })
                .catch((error) => {
                    console.error(error)
                })
        } else {
            button.innerText = "Turn On"
            invoke("plugin:torchlight|torch_on")
                .then((response) => {
                    console.log(response)
                })
                .catch((error) => {
                    console.error(error)
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
        </div>
    </>)
}
