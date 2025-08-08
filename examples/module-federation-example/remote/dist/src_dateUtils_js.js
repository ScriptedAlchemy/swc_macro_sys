"use strict";
exports.ids = ["src_dateUtils_js"];
exports.modules = {
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

};
;