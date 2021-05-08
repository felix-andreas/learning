class Card extends HTMLElement {
    static get observedAttributes() {
        return ["title", "date", "name", "jobTitle"]
    }

    constructor() {
        super()
        this.root = this.attachShadow({ mode: "open" })
        this.root.innerHTML = `
            <style>
            div {
                color: red
            }
            </style>
        `
        this.divTitle = document.createElement("div")
        this.divDate = document.createElement("div")
        this.divName = document.createElement("div")
        this.divJobTitle = document.createElement("div")
        this.root.appendChild(this.divTitle)
        this.root.appendChild(this.divDate)
        this.root.appendChild(this.divName)
        this.root.appendChild(this.divJobTitle)
    }

    attributeChangedCallback(attrName, oldVal, newVal) {
        if (attrName === "title") {
            this.divTitle.innerHTML = newVal
        } else if (attrName === "date") {
            this.divDate.innerHTML = newVal
        } else if (attrName === "name") {
            this.divName.innerHTML = newVal
        } else if (attrName === "jobTitle") {
            this.divJobTitle.innerHTML = newVal
        }
    }
}

window.customElements.define("my-card", Card)
