package io.nordnotes.controllers;

import io.nordnotes.dto.ResultDto;
import io.nordnotes.dto.SystemInfoDto;
import io.nordnotes.dto.SystemInfoDtoFactory;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import static io.nordnotes.controllers.RequestMappings.*;

/**
 * Controller for system operations.
 */
@Slf4j
@RestController
@RequestMapping(M_NORDNOTES)
public class SystemController extends BaseController {

  /** Factory for system information DTOs. */
  private final SystemInfoDtoFactory systemInfoDtoFactory;

  /**
   * Creates system information controller.
   * @param systemInfoDtoFactory System information DTO factory.
   */
  public SystemController(SystemInfoDtoFactory systemInfoDtoFactory) {
    this.systemInfoDtoFactory = systemInfoDtoFactory;
  }

  /**
   * Returns system information.
   *
   * @return System information DTO wrapped with result DTO.
   */
  @GetMapping(M_SYSTEM_INFO)
  public ResultDto<SystemInfoDto> getSystemInfo() {
    return systemInfoDtoFactory.from("nordnotes","1.0.0","Copyright © 2022 Dariusz Depta Engos Software");
  }
}
