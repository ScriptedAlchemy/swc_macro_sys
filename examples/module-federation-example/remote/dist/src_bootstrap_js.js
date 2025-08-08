"use strict";
exports.ids = ["src_bootstrap_js"];
exports.modules = {
"./src/Button.js": 
/*!***********************!*\
  !*** ./src/Button.js ***!
  \***********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (Button)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);

function Button({ text, onClick }) {
    return /*#__PURE__*/ React.createElement("button", {
        onClick: onClick,
        style: {
            padding: '10px 20px',
            backgroundColor: '#007acc',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '16px'
        }
    }, (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.capitalize)(text));
}


}),
"./src/bootstrap.js": 
/*!**************************!*\
  !*** ./src/bootstrap.js ***!
  \**************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  activeUsers: () => (activeUsers),
  addDaysToDate: () => (/* reexport safe */ _dateUtils__WEBPACK_IMPORTED_MODULE_3__.addDaysToDate),
  compareDates: () => (/* reexport safe */ _dateUtils__WEBPACK_IMPORTED_MODULE_3__.compareDates),
  createDebouncedFunction: () => (/* reexport safe */ _utils__WEBPACK_IMPORTED_MODULE_2__.createDebouncedFunction),
  filterData: () => (/* reexport safe */ _functionalUtils__WEBPACK_IMPORTED_MODULE_4__.filterData),
  formatDate: () => (/* reexport safe */ _dateUtils__WEBPACK_IMPORTED_MODULE_3__.formatDate),
  formatUserData: () => (/* reexport safe */ _utils__WEBPACK_IMPORTED_MODULE_2__.formatUserData),
  groupBy: () => (/* reexport safe */ lodash_es__WEBPACK_IMPORTED_MODULE_0__.groupBy),
  groupedUsers: () => (groupedUsers),
  mapData: () => (/* reexport safe */ _functionalUtils__WEBPACK_IMPORTED_MODULE_4__.mapData),
  processUserData: () => (/* reexport safe */ _functionalUtils__WEBPACK_IMPORTED_MODULE_4__.processUserData),
  userAges: () => (userAges),
  users: () => (users)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var _Button__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Button */ "./src/Button.js");
/* ESM import */var _utils__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./utils */ "./src/utils.js");
/* ESM import */var _dateUtils__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./dateUtils */ "./src/dateUtils.js");
/* ESM import */var _functionalUtils__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./functionalUtils */ "./src/functionalUtils.js");





// Demo data
const users = [
    {
        name: 'john doe',
        email: 'john@example.com',
        role: 'admin',
        age: 30,
        active: true
    },
    {
        name: 'jane smith',
        email: 'jane@example.com',
        role: 'user',
        age: 25,
        active: true
    },
    {
        name: 'bob wilson',
        email: 'bob@example.com',
        role: 'admin',
        age: 35,
        active: false
    }
];
// Group users by role using lodash-es
const groupedUsers = (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.groupBy)(users, 'role');
// Use date-fns functions
const today = new Date();
const futureDate = (0,_dateUtils__WEBPACK_IMPORTED_MODULE_3__.addDaysToDate)(today, 7);
const formattedToday = (0,_dateUtils__WEBPACK_IMPORTED_MODULE_3__.formatDate)(today);
const formattedFuture = (0,_dateUtils__WEBPACK_IMPORTED_MODULE_3__.formatDate)(futureDate);
// Use ramda functions
const activeUsers = (0,_functionalUtils__WEBPACK_IMPORTED_MODULE_4__.processUserData)(users);
const userAges = (0,_functionalUtils__WEBPACK_IMPORTED_MODULE_4__.mapData)((user)=>user.age, users);
console.log('Remote app loaded!');
console.log('Grouped users by role:', groupedUsers);
console.log('Today:', formattedToday);
console.log('Future date:', formattedFuture);
console.log('Active users:', activeUsers);
console.log('User ages:', userAges);
// Create a debounced search function
const debouncedSearch = (0,_utils__WEBPACK_IMPORTED_MODULE_2__.createDebouncedFunction)((query)=>{
    console.log('Searching for:', query);
}, 500);
// Demo the remote app
if (typeof document !== 'undefined') {
    const app = document.getElementById('app');
    if (app) {
        app.innerHTML = `
      <div>
        <h2>Remote App - Lodash-ES Demo</h2>
        <div>
          <h3>Users grouped by role:</h3>
          <pre>${JSON.stringify(groupedUsers, null, 2)}</pre>
        </div>
        <div>
          <h3>Formatted user data:</h3>
          <pre>${JSON.stringify(users.map(_utils__WEBPACK_IMPORTED_MODULE_2__.formatUserData), null, 2)}</pre>
        </div>
        <div>
          <h3>Date operations:</h3>
          <p>Today: ${formattedToday}</p>
          <p>Future (+7 days): ${formattedFuture}</p>
        </div>
        <div>
          <h3>Functional operations:</h3>
          <p>Active users: ${JSON.stringify(activeUsers, null, 2)}</p>
          <p>User ages: ${userAges.join(', ')}</p>
        </div>
        <input id="search" placeholder="Type to search (debounced)..." style="padding: 8px; margin: 10px 0; width: 300px;" />
      </div>
    `;
        // Add event listener for debounced search
        const searchInput = document.getElementById('search');
        if (searchInput) {
            searchInput.addEventListener('input', (e)=>{
                debouncedSearch(e.target.value);
            });
        }
    }
}
// Export for testing



}),
"./src/dateUtils.js": 
/*!**************************!*\
  !*** ./src/dateUtils.js ***!
  \**************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  addDaysToDate: () => (addDaysToDate),
  compareDates: () => (compareDates),
  formatDate: () => (formatDate),
  parseDate: () => (parseDate),
  subtractDaysFromDate: () => (subtractDaysFromDate)
});
/* ESM import */var date_fns__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! date-fns */ "webpack/sharing/consume/default/date-fns/date-fns");
/* ESM import */var date_fns__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(date_fns__WEBPACK_IMPORTED_MODULE_0__);

// Export functions that use only a subset of date-fns
const formatDate = (date, formatStr = 'yyyy-MM-dd')=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.format)(date, formatStr);
};
const parseDate = (dateString)=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.parseISO)(dateString);
};
const addDaysToDate = (date, days)=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.addDays)(date, days);
};
const subtractDaysFromDate = (date, days)=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.subDays)(date, days);
};
const compareDates = (date1, date2)=>{
    return {
        isDate1After: (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.isAfter)(date1, date2),
        isDate1Before: (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.isBefore)(date1, date2)
    };
};
// Some internal functions that shouldn't be exposed
const internalFormatter = (date)=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.format)(date, 'dd/MM/yyyy HH:mm:ss');
};
const internalParser = (str)=>{
    return (0,date_fns__WEBPACK_IMPORTED_MODULE_0__.parseISO)(str);
};


}),
"./src/functionalUtils.js": 
/*!********************************!*\
  !*** ./src/functionalUtils.js ***!
  \********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  composeTransforms: () => (composeTransforms),
  createCurriedFunction: () => (createCurriedFunction),
  filterData: () => (filterData),
  getNestedProperty: () => (getNestedProperty),
  getProperty: () => (getProperty),
  mapData: () => (mapData),
  pipeTransforms: () => (pipeTransforms),
  processUserData: () => (processUserData),
  reduceData: () => (reduceData),
  sumValues: () => (sumValues)
});
/* ESM import */var ramda__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! ramda */ "webpack/sharing/consume/default/ramda/ramda");
/* ESM import */var ramda__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(ramda__WEBPACK_IMPORTED_MODULE_0__);
function _define_property(obj, key, value) {
    if (key in obj) {
        Object.defineProperty(obj, key, {
            value: value,
            enumerable: true,
            configurable: true,
            writable: true
        });
    } else {
        obj[key] = value;
    }
    return obj;
}
function _object_spread(target) {
    for(var i = 1; i < arguments.length; i++){
        var source = arguments[i] != null ? arguments[i] : {};
        var ownKeys = Object.keys(source);
        if (typeof Object.getOwnPropertySymbols === "function") {
            ownKeys = ownKeys.concat(Object.getOwnPropertySymbols(source).filter(function(sym) {
                return Object.getOwnPropertyDescriptor(source, sym).enumerable;
            }));
        }
        ownKeys.forEach(function(key) {
            _define_property(target, key, source[key]);
        });
    }
    return target;
}
function ownKeys(object, enumerableOnly) {
    var keys = Object.keys(object);
    if (Object.getOwnPropertySymbols) {
        var symbols = Object.getOwnPropertySymbols(object);
        if (enumerableOnly) {
            symbols = symbols.filter(function(sym) {
                return Object.getOwnPropertyDescriptor(object, sym).enumerable;
            });
        }
        keys.push.apply(keys, symbols);
    }
    return keys;
}
function _object_spread_props(target, source) {
    source = source != null ? source : {};
    if (Object.getOwnPropertyDescriptors) {
        Object.defineProperties(target, Object.getOwnPropertyDescriptors(source));
    } else {
        ownKeys(Object(source)).forEach(function(key) {
            Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key));
        });
    }
    return target;
}

// Export functions that use only a subset of ramda
const composeTransforms = (...fns)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.compose)(...fns);
const pipeTransforms = (...fns)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.pipe)(...fns);
const createCurriedFunction = (fn)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.curry)(fn);
const mapData = (fn, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.map)(fn, data);
const filterData = (predicate, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.filter)(predicate, data);
const reduceData = (reducer, initial, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.reduce)(reducer, initial, data);
const getProperty = (propName, obj)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.prop)(propName, obj);
const getNestedProperty = (pathArray, obj)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.path)(pathArray, obj);
// Example usage functions
const processUserData = (0,ramda__WEBPACK_IMPORTED_MODULE_0__.pipe)((0,ramda__WEBPACK_IMPORTED_MODULE_0__.filter)((user)=>user.active), (0,ramda__WEBPACK_IMPORTED_MODULE_0__.map)((user)=>_object_spread_props(_object_spread({}, user), {
        displayName: user.name.toUpperCase()
    })));
const sumValues = (0,ramda__WEBPACK_IMPORTED_MODULE_0__.reduce)((acc, val)=>acc + val, 0);


}),
"./src/utils.js": 
/*!**********************!*\
  !*** ./src/utils.js ***!
  \**********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  createDebouncedFunction: () => (createDebouncedFunction),
  createThrottledFunction: () => (createThrottledFunction),
  formatUserData: () => (formatUserData),
  omitFields: () => (omitFields),
  pickFields: () => (pickFields)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);
function _define_property(obj, key, value) {
    if (key in obj) {
        Object.defineProperty(obj, key, {
            value: value,
            enumerable: true,
            configurable: true,
            writable: true
        });
    } else {
        obj[key] = value;
    }
    return obj;
}
function _object_spread(target) {
    for(var i = 1; i < arguments.length; i++){
        var source = arguments[i] != null ? arguments[i] : {};
        var ownKeys = Object.keys(source);
        if (typeof Object.getOwnPropertySymbols === "function") {
            ownKeys = ownKeys.concat(Object.getOwnPropertySymbols(source).filter(function(sym) {
                return Object.getOwnPropertyDescriptor(source, sym).enumerable;
            }));
        }
        ownKeys.forEach(function(key) {
            _define_property(target, key, source[key]);
        });
    }
    return target;
}
function ownKeys(object, enumerableOnly) {
    var keys = Object.keys(object);
    if (Object.getOwnPropertySymbols) {
        var symbols = Object.getOwnPropertySymbols(object);
        if (enumerableOnly) {
            symbols = symbols.filter(function(sym) {
                return Object.getOwnPropertyDescriptor(object, sym).enumerable;
            });
        }
        keys.push.apply(keys, symbols);
    }
    return keys;
}
function _object_spread_props(target, source) {
    source = source != null ? source : {};
    if (Object.getOwnPropertyDescriptors) {
        Object.defineProperties(target, Object.getOwnPropertyDescriptors(source));
    } else {
        ownKeys(Object(source)).forEach(function(key) {
            Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key));
        });
    }
    return target;
}

const createDebouncedFunction = (fn, delay = 300)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.debounce)(fn, delay);
};
const createThrottledFunction = (fn, delay = 100)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.throttle)(fn, delay);
};
const pickFields = (obj, fields)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.pick)(obj, fields);
};
const omitFields = (obj, fields)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.omit)(obj, fields);
};
const formatUserData = (userData)=>{
    const publicFields = pickFields(userData, [
        'name',
        'email',
        'role'
    ]);
    return _object_spread_props(_object_spread({}, publicFields), {
        displayName: publicFields.name ? publicFields.name.toUpperCase() : 'Anonymous'
    });
};


}),

};
;