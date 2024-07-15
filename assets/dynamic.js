// Listen for the Enter key being pressed
document.getElementById('code-input').addEventListener('keydown', function(event) {
  if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault(); 
      newCard(); 
  }
});

  
  var globalCardCount = 0;
  var runningInference = false;
  
  function newCard() {
    if (runningInference) {
      console.log("Already running inference, please wait...");
      setAlert("Already running inference, please wait...");
      return;
    }
    var inputElement = document.getElementById("code-input");
    var input = inputElement.value;
    if (input === "") {
      console.log("Please enter a input to analyze");
      setAlert("Please enter a input to analyze");
      return;
    }
    inputElement.value = "";
  
    var cardIndex = globalCardCount;
    globalCardCount++;
    var newCard = document.createElement("div");
    newCard.id = "card-" + cardIndex;
    newCard.innerHTML = `
      <div class="card bg-base-100 shadow-xl w-full">
          <div class="m-4 flex flex-col gap-2">
              <div>${input}</div>
              <div class="flex flex-row justify-end">
                  <span class="loading loading-dots loading-sm"></span>
              </div>
          </div>
      </div>
      `;
    document.getElementById("code-input").before(newCard);
  
    console.log("Running inference on input: " + input);
    runningInference = true;
    fetch("/backend", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ input: input }),
    })
      .then((response) => response)
      .then((data) => {
        console.log(data);
        code_result = data.text();
        return code_result;
      })
      .then((code_result)=>
        updateCard(cardIndex, input, code_result)
      )
      .catch((error) => {
        console.log(error);
      });
  }
  
  function updateCard(cardIndex, input, code_result) {
    var cardElement = document.getElementById("card-" + cardIndex);
    // Updated innerHTML to include better handling for code overflow
    cardElement.innerHTML = `
      <div class="card bg-base-100 shadow-xl w-full">
        <div class="m-4 flex flex-col gap-2">
          <div class="font-bold">Input:</div>
          <div class="p-3 bg-gray-100 rounded overflow-hidden">${input}</div>
          <div class="font-bold mt-4">Result:</div>
          <pre class="p-3 bg-gray-100 text-green-600 rounded overflow-auto max-h-60 custom-code"><code>${code_result}</code></pre>
        </div>
      </div>
    `;
    runningInference = false;
}

  
  function setAlert(msg) {
    var alertElement = document.getElementById("alert");
    alertElement.innerHTML = `
      <div class="alert alert-error">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          <span class="text-error-content">${msg}</span>
      </div>
      `;
    setTimeout(function() {
      alertElement.innerHTML = "";
    }, 3000);
  }