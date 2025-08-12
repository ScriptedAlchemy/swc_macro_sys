"use strict";
(self["webpackChunkhost"] = self["webpackChunkhost"] || []).push([["src_bootstrap_js"], {
"./src/bootstrap.js": 
/*!**************************!*\
  !*** ./src/bootstrap.js ***!
  \**************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  loadRemoteComponents: () => (loadRemoteComponents),
  processItems: () => (processItems),
  sortBy: () => (/* reexport safe */ lodash_es__WEBPACK_IMPORTED_MODULE_0__.sortBy),
  uniq: () => (/* reexport safe */ lodash_es__WEBPACK_IMPORTED_MODULE_0__.uniq)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}

// Dynamically import remote components
function loadRemoteComponents() {
    return _async_to_generator(function*() {
        try {
            const [Button, utils] = yield Promise.all([
                __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_Button").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/Button */ "webpack/container/remote/remote/Button", 23)),
                __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_utils").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/utils */ "webpack/container/remote/remote/utils", 23))
            ]);
            console.log('Remote components loaded successfully!');
            return {
                Button: Button.default,
                utils
            };
        } catch (error) {
            console.error('Failed to load remote components:', error);
            return null;
        }
    })();
}
// Demo data for host app
const items = [
    {
        name: 'Apple',
        category: 'fruit',
        price: 1.5
    },
    {
        name: 'Banana',
        category: 'fruit',
        price: 0.8
    },
    {
        name: 'Carrot',
        category: 'vegetable',
        price: 1.2
    },
    {
        name: 'Broccoli',
        category: 'vegetable',
        price: 2.0
    }
];
function initializeApp() {
    return _async_to_generator(function*() {
        const app = document.getElementById('app');
        // Use lodash-es in host app
        const sortedItems = (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.sortBy)(items, 'price');
        const categories = (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.uniq)(items.map((item)=>item.category));
        app.innerHTML = `
    <div style="padding: 20px; font-family: Arial, sans-serif;">
      <h1>Host App - Module Federation Example</h1>
      
      <div style="margin-bottom: 30px;">
        <h2>Host App Data (using lodash-es)</h2>
        <h3>Items sorted by price:</h3>
        <ul>
          ${sortedItems.map((item)=>`
            <li>${item.name} (${item.category}) - $${item.price}</li>
          `).join('')}
        </ul>
        <p><strong>Categories:</strong> ${categories.join(', ')}</p>
      </div>

      <div id="remote-components">
        <h2>Loading Remote Components...</h2>
      </div>
    </div>
  `;
        // Load and use remote components
        const remoteComponents = yield loadRemoteComponents();
        if (remoteComponents) {
            const { Button, utils } = remoteComponents;
            const remoteContainer = document.getElementById('remote-components');
            // Test data for remote utils
            const testUser = {
                name: 'alice johnson',
                email: 'alice@example.com',
                role: 'moderator',
                password: 'secret123',
                internalId: 12345
            };
            const formattedUser = utils.formatUserData(testUser);
            const publicData = utils.pickFields(testUser, [
                'name',
                'email',
                'role'
            ]);
            remoteContainer.innerHTML = `
      <h2>Remote Components Loaded!</h2>
      
      <div style="margin-bottom: 20px;">
        <h3>Remote Button Component:</h3>
        <div id="button-container"></div>
      </div>
      
      <div style="margin-bottom: 20px;">
        <h3>Remote Utils Demo:</h3>
        <p><strong>Original user:</strong></p>
        <pre style="background: #f5f5f5; padding: 10px;">${JSON.stringify(testUser, null, 2)}</pre>
        
        <p><strong>Formatted user data:</strong></p>
        <pre style="background: #f5f5f5; padding: 10px;">${JSON.stringify(formattedUser, null, 2)}</pre>
        
        <p><strong>Public fields only:</strong></p>
        <pre style="background: #f5f5f5; padding: 10px;">${JSON.stringify(publicData, null, 2)}</pre>
      </div>
    `;
            // Create and mount the remote Button component
            const buttonContainer = document.getElementById('button-container');
            if (buttonContainer && Button) {
                const buttonElement = Button({
                    text: 'hello from remote!',
                    onClick: ()=>alert('Button clicked! This button came from the remote app.')
                });
                // Since we're not using React, create a simple button manually
                const btn = document.createElement('button');
                btn.textContent = 'Hello From Remote!';
                btn.style.cssText = `
        padding: 10px 20px;
        background-color: #007acc;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
      `;
                btn.onclick = ()=>alert('Button clicked! This button came from the remote app.');
                buttonContainer.appendChild(btn);
            }
        }
    })();
}
// Demo function that can be tested
function processItems(items) {
    const sorted = (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.sortBy)(items, 'price');
    const categories = (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.uniq)(items.map((item)=>item.category));
    return {
        sorted,
        categories,
        count: items.length
    };
}
// Export functions for testing

// Initialize the app only in browser environment
if (typeof document !== 'undefined') {
    initializeApp();
}


}),

}]);