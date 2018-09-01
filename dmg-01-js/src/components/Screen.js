"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    }
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
exports.__esModule = true;
var React = require("react");
var ReactDOM = require("react-dom");
var Screen = /** @class */ (function (_super) {
    __extends(Screen, _super);
    function Screen(props) {
        var _this = _super.call(this, props) || this;
        _this.drawCanvas = function (data) {
            var ctx = _this.getCtx();
            if (!ctx) {
                return;
            }
            ctx.putImageData(data, 0, 0);
        };
        _this.state = {};
        return _this;
    }
    Screen.prototype.componentWillUnmount = function () {
        this.setState({ ctx: undefined });
    };
    Screen.prototype.getCtx = function () {
        var ctx = this.state.ctx;
        if (ctx) {
            return ctx;
        }
        var canvas = ReactDOM.findDOMNode(this.refs.screen);
        if (canvas === null) {
            return undefined;
        }
        var newCtx = canvas.getContext('2d') || undefined;
        this.setState({ ctx: newCtx });
        return newCtx;
    };
    Screen.prototype.canvas = function () {
        return React.createElement("canvas", { height: 144, width: 160, id: "screen", ref: "screen" });
    };
    Screen.prototype.render = function () {
        return (React.createElement("div", { className: "screen" }, this.canvas()));
    };
    return Screen;
}(React.Component));
exports["default"] = Screen;
